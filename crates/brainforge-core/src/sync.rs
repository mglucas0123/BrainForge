use std::fs;
use std::path::Path;

use anyhow::{Context, Result};

use crate::adapter::Adapter;
use crate::copy_util::{self, ensure_dir, mirror_dir_contents};
use crate::kit::KitPaths;
use crate::memory::sync_memory_to_cursor;

pub fn run_sync(paths: &KitPaths, adapters: &[Adapter], embed_commands: bool) -> Result<()> {
    for adapter in adapters {
        match adapter {
            Adapter::Cursor => sync_cursor(paths, embed_commands)?,
            Adapter::Copilot => sync_copilot(paths)?,
            Adapter::Antigravity => sync_antigravity(paths)?,
        }
    }
    sync_agents_md(paths)?;
    Ok(())
}

/// `.cursor/` = bridge only (rules + commands + memory link). Kit stays in `.brainforge/`.
fn sync_cursor(paths: &KitPaths, embed_commands: bool) -> Result<()> {
    let cursor = paths.project_root.join(".cursor");
    let core = paths.core();

    ensure_dir(&cursor)?;
    prune_legacy_cursor_mirror(&cursor)?;

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

    println!("[cursor] bridge → .cursor/ (rules + commands + project); kit em .brainforge/");
    Ok(())
}

/// Drop mirror/junction clutter — skills and docs live only under `.brainforge/`.
fn prune_legacy_cursor_mirror(cursor: &Path) -> Result<()> {
    for name in ["skills", "skills-optional", "docs", "hooks.example"] {
        clear_path_for_link(&cursor.join(name))?;
    }
    for name in ["skills-catalog.json", "installed-skills.json"] {
        clear_path_for_link(&cursor.join(name))?;
    }
    Ok(())
}

fn clear_path_for_link(path: &Path) -> Result<()> {
    if !path.exists() {
        return Ok(());
    }
    if path.is_symlink() {
        fs::remove_file(path).with_context(|| format!("remove link {}", path.display()))?;
    } else if path.is_dir() {
        fs::remove_dir_all(path).with_context(|| format!("remove dir {}", path.display()))?;
    } else {
        fs::remove_file(path).with_context(|| format!("remove file {}", path.display()))?;
    }
    Ok(())
}

fn link_or_copy_cursor_memory(paths: &KitPaths) -> Result<()> {
    let link = paths.project_root.join(".cursor").join("project");
    let target = paths
        .memory()
        .canonicalize()
        .with_context(|| "canonicalize .brainforge/memory")?;

    clear_path_for_link(&link)?;
    if let Some(parent) = link.parent() {
        ensure_dir(parent)?;
    }

    #[cfg(windows)]
    {
        if std::os::windows::fs::symlink_dir(&target, &link).is_ok() {
            println!("[cursor] .cursor/project → .brainforge/memory (symlink)");
            return Ok(());
        }
        if windows_directory_junction(&link, &target) {
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

#[cfg(windows)]
fn windows_directory_junction(link: &Path, target: &Path) -> bool {
    use std::process::Command;

    Command::new("cmd")
        .args([
            "/C",
            "mklink",
            "/J",
            &link.to_string_lossy(),
            &target.to_string_lossy(),
        ])
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
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
| `.cursor/project/` | Link → `.brainforge/memory/` |

Skills, docs, hooks: **only** under `.brainforge/` (not mirrored here).

To refresh: `brainforge sync`
"#;
    fs::write(&readme, text).with_context(|| format!("write {}", readme.display()))?;
    Ok(())
}

fn write_cursor_generated_marker(paths: &KitPaths) -> Result<()> {
    let marker = paths
        .project_root
        .join(".cursor")
        .join(".brainforge-generated");
    let text = "BrainForge bridge. Canonical kit: .brainforge/\n";
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
        copy_util::copy_file(&src, &github.join("copilot-instructions.md"))?;
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
    let text = "BrainForge bridge. Canonical: .brainforge/adapters/antigravity/\n";
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
    println!("[host] AGENTS.md (bridge — created)");
    Ok(())
}
