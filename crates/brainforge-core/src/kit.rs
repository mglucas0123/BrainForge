use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};

/// Host kit folder (hidden, canonical edit location).
pub const KIT_DIR: &str = ".brainforge";
/// Legacy kit folder name (BrainForge dev repo + old hosts).
pub const KIT_DIR_LEGACY: &str = "brainforge";

/// Resolved layout under the repo / host project.
pub struct KitPaths {
    /// Project root (where .cursor / .github are written).
    pub project_root: PathBuf,
    /// `.brainforge/` or legacy `brainforge/` kit directory.
    pub kit_root: PathBuf,
}

impl KitPaths {
    pub fn resolve(project_root: impl AsRef<Path>, kit_override: Option<&Path>) -> Result<Self> {
        let project_root = project_root
            .as_ref()
            .canonicalize()
            .with_context(|| "resolve project root")?;

        let kit_root = if let Some(k) = kit_override {
            k.canonicalize()
                .with_context(|| format!("kit path {}", k.display()))?
        } else if let Ok(env_kit) = env::var("BRAINFORGE_KIT") {
            PathBuf::from(env_kit)
                .canonicalize()
                .with_context(|| "BRAINFORGE_KIT")?
        } else {
            discover_kit(&project_root)?
        };

        validate_kit(&kit_root)?;
        Ok(Self {
            project_root,
            kit_root,
        })
    }

    pub fn core(&self) -> PathBuf {
        self.kit_root.join("core")
    }

    pub fn memory(&self) -> PathBuf {
        self.kit_root.join("memory")
    }

    pub fn adapters(&self) -> PathBuf {
        self.kit_root.join("adapters")
    }

    pub fn rtk_exe(&self) -> PathBuf {
        self.kit_root.join("tools").join("rtk").join("rtk.exe")
    }
}

pub fn validate_kit(kit_root: &Path) -> Result<()> {
    let marker = kit_root.join("core").join("BRAINFORGE.md");
    if !marker.is_file() {
        bail!(
            "invalid kit at {}: missing core/BRAINFORGE.md",
            kit_root.display()
        );
    }
    Ok(())
}

fn is_kit_root(path: &Path) -> bool {
    path.join("core").join("BRAINFORGE.md").is_file()
}

/// Drop duplicate `brainforge/` when canonical `.brainforge/` exists (v1.0.0 init side-effect).
pub fn remove_legacy_host_kit(project_root: &Path) -> Result<bool> {
    let modern = project_root.join(KIT_DIR);
    let legacy = project_root.join(KIT_DIR_LEGACY);

    if !is_kit_root(&modern) || !legacy.is_dir() || !is_kit_root(&legacy) {
        return Ok(false);
    }

    if let (Ok(m), Ok(l)) = (modern.canonicalize(), legacy.canonicalize()) {
        if m == l {
            return Ok(false);
        }
    }

    fs::remove_dir_all(&legacy).with_context(|| format!("remove legacy {}", legacy.display()))?;
    eprintln!(
        "[brainforge] removed legacy {}/ (canonical: {}/)",
        KIT_DIR_LEGACY, KIT_DIR
    );
    Ok(true)
}

/// Walk parents from `start` looking for `.brainforge/` then legacy `brainforge/`.
pub fn discover_kit(start: &Path) -> Result<PathBuf> {
    let mut cur = start
        .canonicalize()
        .with_context(|| format!("canonicalize {}", start.display()))?;

    loop {
        for name in [KIT_DIR, KIT_DIR_LEGACY] {
            let nested = cur.join(name);
            if is_kit_root(&nested) {
                return Ok(nested);
            }
        }
        if is_kit_root(&cur) {
            return Ok(cur);
        }
        if !cur.pop() {
            break;
        }
    }

    bail!(
        "kit not found from {} (expected {}/ or {}/). Set BRAINFORGE_KIT or --kit",
        start.display(),
        KIT_DIR,
        KIT_DIR_LEGACY
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn remove_legacy_host_kit_drops_duplicate() {
        let root = tempfile::tempdir().unwrap();
        let modern = root.path().join(KIT_DIR);
        let legacy = root.path().join(KIT_DIR_LEGACY);
        for k in [&modern, &legacy] {
            fs::create_dir_all(k.join("core")).unwrap();
            fs::write(k.join("core/BRAINFORGE.md"), "x").unwrap();
        }
        assert!(remove_legacy_host_kit(root.path()).unwrap());
        assert!(!legacy.exists());
        assert!(modern.is_dir());
    }

    #[test]
    fn remove_legacy_host_kit_noop_without_modern() {
        let root = tempfile::tempdir().unwrap();
        let legacy = root.path().join(KIT_DIR_LEGACY);
        fs::create_dir_all(legacy.join("core")).unwrap();
        fs::write(legacy.join("core/BRAINFORGE.md"), "x").unwrap();
        assert!(!remove_legacy_host_kit(root.path()).unwrap());
        assert!(legacy.is_dir());
    }
}
