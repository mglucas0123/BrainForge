use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};

use crate::adapter::Adapter;
use crate::config::write_config_install;
use crate::copy_util;
use crate::doctor::{self, DoctorReport};
use crate::install::{InstallOptions, run_install};
use crate::kit::{KitPaths, discover_kit, validate_kit};
use crate::sync::run_sync;

#[derive(Debug, Clone, Default)]
pub struct InitOptions {
    pub force_kit: bool,
    pub copy_exe: bool,
    pub embed_commands: bool,
}

#[derive(Debug, Clone)]
pub struct InitReport {
    pub kit_installed: bool,
    pub exe_copied: bool,
    pub config_updated: bool,
    pub adapters_synced: Vec<Adapter>,
    pub doctor: DoctorReport,
}

/// Bootstrap a host project: copy kit (if needed), optional exe, sync adapters, doctor.
pub fn run_init(
    source_kit: &Path,
    target_project: &Path,
    exe_source: Option<&Path>,
    adapters: &[Adapter],
    opts: InitOptions,
) -> Result<InitReport> {
    if adapters.is_empty() {
        bail!("nenhum adaptador selecionado");
    }

    let target_project = target_project
        .canonicalize()
        .with_context(|| format!("target project {}", target_project.display()))?;

    let kit_marker = target_project
        .join("brainforge")
        .join("core")
        .join("BRAINFORGE.md");
    let needs_kit = !kit_marker.is_file();

    let mut kit_installed = false;
    if needs_kit || opts.force_kit {
        let install_report = run_install(
            source_kit,
            &target_project,
            None,
            InstallOptions {
                force: opts.force_kit,
                copy_exe: false,
                run_sync: false,
            },
        )?;
        kit_installed = install_report.kit_copied;
    }

    let mut exe_copied = false;
    if opts.copy_exe {
        if let Some(exe) = exe_source {
            if exe.is_file() {
                let dest = target_project.join("brainforge.exe");
                if opts.force_kit || !dest.is_file() {
                    copy_util::copy_file(exe, &dest)?;
                    exe_copied = true;
                }
            } else {
                eprintln!("warn: executable not found at {}", exe.display());
            }
        } else {
            eprintln!("warn: --with-exe omitido: executável atual desconhecido");
        }
    }

    let config_updated = write_config_install(&target_project, adapters)?;

    let paths = KitPaths::resolve(&target_project, None)?;
    run_sync(&paths, adapters, opts.embed_commands)?;

    let doctor = doctor::run_doctor(&paths)?;

    Ok(InitReport {
        kit_installed,
        exe_copied,
        config_updated,
        adapters_synced: adapters.to_vec(),
        doctor,
    })
}

/// Remove adapter outputs produced by `sync` (does not remove `brainforge/` kit).
pub fn run_uninstall(target_project: &Path, adapters: &[Adapter]) -> Result<()> {
    if adapters.is_empty() {
        bail!("nenhum adaptador selecionado");
    }

    let root = target_project
        .canonicalize()
        .with_context(|| format!("project {}", target_project.display()))?;

    for adapter in adapters {
        match adapter {
            Adapter::Cursor => uninstall_cursor(&root)?,
            Adapter::Copilot => uninstall_copilot(&root)?,
            Adapter::Antigravity => uninstall_antigravity(&root)?,
        }
    }

    Ok(())
}

fn uninstall_cursor(root: &Path) -> Result<()> {
    let cursor = root.join(".cursor");
    for rel in [
        "skills",
        "skills-optional",
        "commands",
        "rules",
        "docs",
        "project",
        "hooks.example",
    ] {
        remove_path(&cursor.join(rel))?;
    }
    for name in ["skills-catalog.json", "installed-skills.json"] {
        remove_path(&cursor.join(name))?;
    }
    println!("[cursor] outputs removidos de .cursor/");
    Ok(())
}

fn uninstall_copilot(root: &Path) -> Result<()> {
    let file = root.join(".github").join("copilot-instructions.md");
    if remove_path(&file)? {
        println!("[copilot] removido .github/copilot-instructions.md");
    }
    prune_empty_dir(&root.join(".github"))?;
    Ok(())
}

fn uninstall_antigravity(root: &Path) -> Result<()> {
    let agents = root.join(".agents");
    remove_path(&agents.join("rules"))?;
    remove_path(&agents.join("workflows"))?;
    prune_empty_dir(&agents)?;
    println!("[antigravity] outputs removidos de .agents/");
    Ok(())
}

/// Returns true if something was removed.
fn remove_path(path: &Path) -> Result<bool> {
    if path.is_dir() {
        fs::remove_dir_all(path).with_context(|| format!("remove dir {}", path.display()))?;
        return Ok(true);
    }
    if path.is_file() {
        fs::remove_file(path).with_context(|| format!("remove file {}", path.display()))?;
        return Ok(true);
    }
    Ok(false)
}

fn prune_empty_dir(path: &Path) -> Result<()> {
    if !path.is_dir() {
        return Ok(());
    }
    if fs::read_dir(path)?.next().is_none() {
        fs::remove_dir(path).with_context(|| format!("remove empty dir {}", path.display()))?;
    }
    Ok(())
}

/// Locate kit source for `install` / `init` (exe parents, cwd, env, --kit).
pub fn discover_source_kit(kit_override: Option<&Path>) -> Result<PathBuf> {
    if let Some(k) = kit_override {
        let k = k
            .canonicalize()
            .with_context(|| format!("kit path {}", k.display()))?;
        validate_kit(&k)?;
        return Ok(k);
    }

    if let Ok(env_kit) = std::env::var("BRAINFORGE_KIT") {
        let k = PathBuf::from(env_kit)
            .canonicalize()
            .with_context(|| "BRAINFORGE_KIT")?;
        validate_kit(&k)?;
        return Ok(k);
    }

    if let Ok(cwd) = std::env::current_dir() {
        if let Ok(kit) = discover_kit(&cwd) {
            return Ok(kit);
        }
    }

    if let Ok(exe) = std::env::current_exe() {
        let mut cur = exe.parent();
        while let Some(dir) = cur {
            if let Ok(kit) = discover_kit(dir) {
                return Ok(kit);
            }
            cur = dir.parent();
        }
    }

    bail!(
        "fonte do kit não encontrada. Defina BRAINFORGE_KIT, use --kit <caminho>/brainforge, \
         ou rode a partir do repositório BrainForge / pasta com brainforge/"
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn uninstall_copilot_removes_instructions_only() {
        let dir = tempfile::tempdir().unwrap();
        let github = dir.path().join(".github");
        fs::create_dir_all(&github).unwrap();
        let file = github.join("copilot-instructions.md");
        fs::write(&file, "test").unwrap();

        run_uninstall(dir.path(), &[Adapter::Copilot]).unwrap();
        assert!(!file.exists());
    }
}
