use std::env;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};

/// Resolved layout under the repo / host project.
pub struct KitPaths {
    /// Project root (where .cursor / .github are written).
    pub project_root: PathBuf,
    /// `brainforge/` kit directory (core, memory, adapters, tools).
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

fn validate_kit(kit_root: &Path) -> Result<()> {
    let marker = kit_root.join("core").join("BRAINFORGE.md");
    if !marker.is_file() {
        bail!(
            "invalid kit at {}: missing core/BRAINFORGE.md",
            kit_root.display()
        );
    }
    Ok(())
}

fn discover_kit(start: &Path) -> Result<PathBuf> {
    let mut cur = start
        .canonicalize()
        .with_context(|| format!("canonicalize {}", start.display()))?;

    loop {
        let nested = cur.join("brainforge");
        if nested.join("core").join("BRAINFORGE.md").is_file() {
            return Ok(nested);
        }
        if cur.file_name().is_some_and(|n| n == "brainforge")
            && cur.join("core").join("BRAINFORGE.md").is_file()
        {
            return Ok(cur);
        }
        if !cur.pop() {
            break;
        }
    }

    bail!(
        "brainforge kit not found from {}. Set BRAINFORGE_KIT or use --kit <path>",
        start.display()
    )
}
