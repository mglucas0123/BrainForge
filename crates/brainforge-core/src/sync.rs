use std::fs;
use std::path::Path;

use anyhow::{Context, Result};

use crate::adapter::Adapter;
use crate::config::load_config;
use crate::copy_util::{self, copy_file, copy_tree, ensure_dir, mirror_dir_contents};
use crate::kit::KitPaths;
use crate::memory::sync_memory_to_cursor;
use crate::sync_mode::SyncMode;

pub fn run_sync(paths: &KitPaths, adapters: &[Adapter], embed_commands: bool) -> Result<()> {
    let mode = SyncMode::parse(&load_config(&paths.project_root).sync.mode);
    for adapter in adapters {
        match adapter {
            Adapter::Cursor => sync_cursor(paths, embed_commands, mode)?,
            Adapter::Copilot => sync_copilot(paths)?,
            Adapter::Antigravity => sync_antigravity(paths)?,
        }
    }
    sync_agents_md(paths)?;
    Ok(())
}

fn sync_cursor(paths: &KitPaths, embed_commands: bool, mode: SyncMode) -> Result<()> {
    match mode {
        SyncMode::Thin => sync_cursor_thin(paths, embed_commands),
        SyncMode::Mirror => sync_cursor_mirror(paths, embed_commands),
    }
}

/// Thin bridge: rules + slash commands + memory link only (no skills/docs mirror).
fn sync_cursor_thin(paths: &KitPaths, embed_commands: bool) -> Result<()> {
    let cursor = paths.project_root.join(".cursor");
    let core = paths.core();

    ensure_dir(&cursor)?;
    prune_cursor_mirror_artifacts(&cursor)?;

    ensure_dir(&cursor.join("rules"))?;
    let rules_src = paths.adapters().join("cursor").join("rules");
    if rules_src.is_dir() {
        mirror_dir_contents(&rules_src, &cursor.join("rules"))?;
    }

    ensure_dir(&cursor.join("commands"))?;
    copy_util::clear_dir_children(&cursor.join("commands"))?;
    let commands_src = core.join("commands");
    if commands_src.is_dir() {
        mirror_dir_contents(&commands_src, &cursor.join("commands"))?;
    } else if embed_commands {
        let n = crate::embedded::write_embedded_commands(&cursor.join("commands"))?;
        println!("[cursor] embedded {n} command(s)");
    }

    link_or_copy_cursor_memory(paths)?;

    write_cursor_bridge_readme(paths)?;
    write_cursor_generated_marker(paths)?;

    if !paths.rtk_exe().is_file() {
        eprintln!(
            "warn: RTK missing at {} — run install-rtk-local.ps1",
            paths.rtk_exe().display()
        );
    }

    println!(
        "[cursor] thin bridge → .cursor/ (rules + commands); kit em .brainforge/"
    );
    Ok(())
}

/// Legacy full mirror into `.cursor/`.
fn sync_cursor_mirror(paths: &KitPaths, embed_commands: bool) -> Result<()> {
    let cursor = paths.project_root.join(".cursor");
    let core = paths.core();

    ensure_dir(&cursor)?;
    ensure_dir(&cursor.join("skills"))?;
    ensure_dir(&cursor.join("commands"))?;
    ensure_dir(&cursor.join("rules"))?;
    ensure_dir(&cursor.join("project"))?;
    ensure_dir(&cursor.join("docs"))?;

    let skills_src = core.join("skills");
    if skills_src.is_dir() {
        let skills_dst = cursor.join("skills");
        copy_util::clear_dir_children(&skills_dst)?;
        mirror_dir_contents(&skills_src, &skills_dst)?;
    }

    let optional_src = core.join("skills-optional");
    if optional_src.is_dir() {
        let optional_dst = cursor.join("skills-optional");
        ensure_dir(&optional_dst)?;
        mirror_dir_contents(&optional_src, &optional_dst)?;
    }

    copy_tree(&core.join("docs"), &cursor.join("docs"))?;

    for name in ["skills-catalog.json", "installed-skills.json"] {
        let src = core.join(name);
        if src.is_file() {
            copy_file(&src, &cursor.join(name))?;
        }
    }

    let commands_dst = cursor.join("commands");
    let commands_src = core.join("commands");
    if commands_src.is_dir() {
        mirror_dir_contents(&commands_src, &commands_dst)?;
    } else if embed_commands {
        let n = crate::embedded::write_embedded_commands(&commands_dst)?;
        println!("[cursor] embedded {n} command(s) (kit commands/ missing)");
    }

    sync_cursor_hooks_example(paths)?;

    let rules_src = paths.adapters().join("cursor").join("rules");
    if rules_src.is_dir() {
        mirror_dir_contents(&rules_src, &cursor.join("rules"))?;
    }

    sync_memory_to_cursor(paths)?;

    if !paths.rtk_exe().is_file() {
        eprintln!(
            "warn: RTK missing at {} — run install-rtk-local.ps1",
            paths.rtk_exe().display()
        );
    }

    write_cursor_generated_marker(paths)?;
    println!("[cursor] mirror mode → full .cursor/ copy");
    Ok(())
}

fn prune_cursor_mirror_artifacts(cursor: &Path) -> Result<()> {
    for name in ["skills", "skills-optional", "docs", "hooks.example"] {
        let p = cursor.join(name);
        if p.is_dir() {
            fs::remove_dir_all(&p).with_context(|| format!("remove {}", p.display()))?;
        }
    }
    for name in ["skills-catalog.json", "installed-skills.json"] {
        let p = cursor.join(name);
        if p.is_file() {
            fs::remove_file(&p).with_context(|| format!("remove {}", p.display()))?;
        }
    }
    Ok(())
}

fn link_or_copy_cursor_memory(paths: &KitPaths) -> Result<()> {
    let link = paths.project_root.join(".cursor").join("project");
    let target = paths
        .memory()
        .canonicalize()
        .with_context(|| "canonicalize .brainforge/memory")?;

    if link.exists() {
        if link.is_symlink() {
            fs::remove_file(&link).ok();
        } else if link.is_dir() {
            fs::remove_dir_all(&link).ok();
        }
    }

    if let Some(parent) = link.parent() {
        ensure_dir(parent)?;
    }

    #[cfg(windows)]
    {
        if std::os::windows::fs::symlink_dir(&target, &link).is_ok() {
            println!("[cursor] .cursor/project → .brainforge/memory (junction)");
            return Ok(());
        }
    }

    #[cfg(unix)]
    {
        if std::os::unix::fs::symlink(&target, &link).is_ok() {
            println!("[cursor] .cursor/project → .brainforge/memory (symlink)");
            return Ok(());
        }
    }

    eprintln!("warn: link failed; copying memory into .cursor/project/");
    sync_memory_to_cursor(paths)
}

fn write_cursor_bridge_readme(paths: &KitPaths) -> Result<()> {
    let readme = paths.project_root.join(".cursor").join("README.md");
    let text = r#"# BrainForge (Cursor bridge)

This folder is **not** a second copy of the kit.

| Path | Role |
|------|------|
| `.brainforge/` | **Edit here** — skills, memory, core, adapters |
| `.cursor/rules/` | Cursor always-on rule → points at `.brainforge/` |
| `.cursor/commands/` | Slash commands (`/brainforge`, etc.) |
| `.cursor/project/` | Link or mirror of `.brainforge/memory/` |

Skills live under `.brainforge/core/skills/` — open files from there or let the agent read them via rules.

To refresh bridges: `brainforge sync`
"#;
    fs::write(&readme, text).with_context(|| format!("write {}", readme.display()))?;
    Ok(())
}

fn write_cursor_generated_marker(paths: &KitPaths) -> Result<()> {
    let marker = paths
        .project_root
        .join(".cursor")
        .join(".brainforge-generated");
    let text = "BrainForge thin bridge. Canonical kit: .brainforge/ — do not add skills/docs here.\n";
    copy_util::ensure_dir(marker.parent().unwrap())?;
    fs::write(&marker, text)
        .with_context(|| format!("write {}", marker.display()))?;
    Ok(())
}

fn sync_copilot(paths: &KitPaths) -> Result<()> {
    let github = paths.project_root.join(".github");
    ensure_dir(&github)?;
    let src = paths
        .adapters()
        .join("copilot")
        .join("copilot-instructions.md");
    if src.is_file() {
        copy_file(&src, &github.join("copilot-instructions.md"))?;
        println!("[copilot] bridge → .github/copilot-instructions.md");
    }
    Ok(())
}

fn sync_antigravity(paths: &KitPaths) -> Result<()> {
    let agents = paths.project_root.join(".agents");
    ensure_dir(&agents.join("rules"))?;
    ensure_dir(&agents.join("workflows"))?;

    let rules_src = paths.adapters().join("antigravity").join("rules");
    if rules_src.is_dir() {
        mirror_dir_contents(&rules_src, &agents.join("rules"))?;
    }

    let wf_src = paths.adapters().join("antigravity").join("workflows");
    if wf_src.is_dir() {
        mirror_dir_contents(&wf_src, &agents.join("workflows"))?;
    }

    let marker = paths.project_root.join(".agents").join(".brainforge-generated");
    let text = "BrainForge bridge. Canonical: .brainforge/adapters/antigravity/ — rules/workflows here are entry points only.\n";
    if let Some(parent) = marker.parent() {
        ensure_dir(parent)?;
        fs::write(&marker, text).ok();
    }
    println!("[antigravity] bridge → .agents/ (rules + workflows)");
    Ok(())
}

fn sync_agents_md(paths: &KitPaths) -> Result<()> {
    let src = paths.adapters().join("AGENTS.md");
    if !src.is_file() {
        return Ok(());
    }
    let dest = paths.project_root.join("AGENTS.md");
    if dest.is_file() {
        return Ok(());
    }
    copy_util::copy_file(&src, &dest)?;
    println!("[host] AGENTS.md (thin bridge — created)");
    Ok(())
}

fn sync_cursor_hooks_example(paths: &KitPaths) -> Result<()> {
    let example = paths
        .adapters()
        .join("cursor")
        .join("hooks.example");
    if !example.is_dir() {
        return Ok(());
    }
    let dest = paths.project_root.join(".cursor").join("hooks.example");
    copy_util::mirror_dir_contents(&example, &dest)?;
    println!("[cursor] .cursor/hooks.example/ (opt-in — see .brainforge/core/docs/CURSOR-HOOKS.md)");
    Ok(())
}
