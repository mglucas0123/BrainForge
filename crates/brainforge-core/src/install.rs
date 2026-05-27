use std::fs;
use std::path::Path;

use anyhow::{Context, Result, bail};

use crate::adapter::Adapter;
use crate::config::{self, write_default_config_if_missing};
use crate::copy_util::{copy_file, copy_tree, ensure_dir};
use crate::kit::{KitPaths, KIT_DIR};
use crate::sync::run_sync;

const MEMORY_FILES: [&str; 2] = [".context.md", ".user.md"];

#[derive(Debug, Clone, Default)]
pub struct InstallOptions {
    pub force: bool,
    pub copy_exe: bool,
    pub run_sync: bool,
}

#[derive(Debug, Clone, Default)]
pub struct InstallReport {
    pub kit_copied: bool,
    pub exe_copied: bool,
    pub config_created: bool,
    pub sync_ran: bool,
    pub adapters_synced: Vec<Adapter>,
}

/// Copy kit from `source_kit` into `<target_project>/.brainforge/`, optional exe + config + sync.
pub fn run_install(
    source_kit: &Path,
    target_project: &Path,
    exe_source: Option<&Path>,
    opts: InstallOptions,
) -> Result<InstallReport> {
    let target_project = target_project
        .canonicalize()
        .with_context(|| format!("target project {}", target_project.display()))?;
    let source_kit = source_kit
        .canonicalize()
        .with_context(|| format!("source kit {}", source_kit.display()))?;

    let dest_kit = target_project.join(KIT_DIR);
    if dest_kit.exists() {
        if opts.force {
            fs::remove_dir_all(&dest_kit)
                .with_context(|| format!("remove {}", dest_kit.display()))?;
        } else {
            bail!(
                "kit already exists at {}; use --force to replace",
                dest_kit.display()
            );
        }
    }

    copy_kit_tree(&source_kit, &dest_kit, opts.force)?;
    patch_host_kit_paths(&dest_kit)?;
    let kit_copied = true;

    let exe_copied = if opts.copy_exe {
        copy_install_exe(exe_source, &target_project)?
    } else {
        false
    };

    let config_created = write_default_config_if_missing(&target_project)?;

    let cfg = config::load_config(&target_project);
    if cfg.brainforge.memory_dir != config::expected_memory_dir() {
        eprintln!(
            "warn: brainforge.memory_dir = '{}' (expected '{}')",
            cfg.brainforge.memory_dir,
            config::expected_memory_dir()
        );
    }

    let mut report = InstallReport {
        kit_copied,
        exe_copied,
        config_created,
        sync_ran: false,
        adapters_synced: vec![],
    };

    if opts.run_sync {
        let adapters = cfg.install.enabled_adapters();
        if adapters.is_empty() {
            eprintln!("warn: no adapters enabled in brainforge.toml [install]; skipping sync");
        } else {
            let paths = KitPaths {
                project_root: target_project.clone(),
                kit_root: dest_kit,
            };
            run_sync(&paths, &adapters, false)?;
            report.sync_ran = true;
            report.adapters_synced = adapters;
        }
    }

    Ok(report)
}

fn copy_kit_tree(src: &Path, dst: &Path, force: bool) -> Result<()> {
    ensure_dir(dst)?;
    for entry in fs::read_dir(src).with_context(|| format!("read {}", src.display()))? {
        let entry = entry?;
        let name = entry.file_name();
        let from = entry.path();
        let to = dst.join(&name);

        if from.is_dir() {
            if name == "memory" {
                copy_memory_dir(&from, &to, force)?;
            } else {
                copy_tree(&from, &to)?;
            }
        } else if should_copy_file(&to, force) {
            copy_file(&from, &to)?;
        }
    }
    Ok(())
}

fn copy_memory_dir(src: &Path, dst: &Path, force: bool) -> Result<()> {
    ensure_dir(dst)?;
    for entry in fs::read_dir(src).with_context(|| format!("read {}", src.display()))? {
        let entry = entry?;
        let name = entry.file_name();
        let name_str = name.to_string_lossy();
        let from = entry.path();
        let to = dst.join(&name);

        if from.is_dir() {
            copy_tree(&from, &to)?;
        } else if MEMORY_FILES.iter().any(|m| *m == name_str) {
            if should_copy_file(&to, force) {
                copy_file(&from, &to)?;
            } else {
                eprintln!("skip: preserve existing {}", to.display());
            }
        } else if should_copy_file(&to, force) {
            copy_file(&from, &to)?;
        }
    }
    Ok(())
}

fn should_copy_file(dest: &Path, force: bool) -> bool {
    force || !dest.is_file()
}

/// Rewrite legacy `brainforge/...` path strings in copied kit docs for the host layout.
fn patch_host_kit_paths(kit_root: &Path) -> Result<()> {
    use std::ffi::OsStr;
    use walkdir::WalkDir;

    let replacements = [
        ("brainforge/memory", ".brainforge/memory"),
        ("brainforge/core", ".brainforge/core"),
        ("brainforge/tools", ".brainforge/tools"),
        ("brainforge/adapters", ".brainforge/adapters"),
    ];

    for entry in WalkDir::new(kit_root)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        let path = entry.path();
        let ext = path.extension().and_then(OsStr::to_str).unwrap_or("");
        if !matches!(ext, "md" | "mdc" | "toml" | "json" | "ps1" | "txt") {
            continue;
        }
        let Ok(mut text) = fs::read_to_string(path) else {
            continue;
        };
        let original = text.clone();
        for (from, to) in replacements {
            text = text.replace(from, to);
        }
        if text != original {
            fs::write(path, text)
                .with_context(|| format!("patch paths in {}", path.display()))?;
        }
    }
    Ok(())
}

fn copy_install_exe(exe_source: Option<&Path>, target_project: &Path) -> Result<bool> {
    let Some(exe) = exe_source else {
        eprintln!("warn: --with-exe set but current executable unknown");
        return Ok(false);
    };
    if !exe.is_file() {
        eprintln!("warn: executable not found at {}", exe.display());
        return Ok(false);
    }
    let dest = target_project.join("brainforge.exe");
    copy_file(exe, &dest)?;
    Ok(true)
}

/// JSON snippet for `.cursor/mcp.json` (project-relative `cwd`).
pub fn format_mcp_config_json(project_root: &Path, exe: &Path) -> Result<String> {
    let project_root = project_root
        .canonicalize()
        .with_context(|| "canonicalize project root for MCP config")?;
    let exe = exe
        .canonicalize()
        .with_context(|| format!("canonicalize exe {}", exe.display()))?;

    let command = exe.to_string_lossy().replace('\\', "\\\\");
    let cwd = project_root.to_string_lossy().replace('\\', "\\\\");

    Ok(format!(
        r#"{{
  "mcpServers": {{
    "brainforge": {{
      "command": "{command}",
      "args": ["mcp"],
      "cwd": "{cwd}"
    }}
  }}
}}
"#
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn mcp_config_json_escapes_paths() {
        let dir = tempfile::tempdir().unwrap();
        let exe = dir.path().join("brainforge.exe");
        let mut f = fs::File::create(&exe).unwrap();
        writeln!(f, "stub").unwrap();

        let json = format_mcp_config_json(dir.path(), &exe).unwrap();
        assert!(json.contains("\"mcp\""));
        assert!(json.contains("brainforge.exe"));
    }
}
