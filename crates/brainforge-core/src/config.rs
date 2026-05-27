use std::path::Path;

use anyhow::{Context, Result};
use serde::Deserialize;

use crate::adapter::Adapter;

pub const DEFAULT_CONFIG_TEMPLATE: &str = r#"# BrainForge host config
# Docs: brainforge/HOST-SETUP.md

[brainforge]
version = "1"
memory_dir = "brainforge/memory"
caveman_level = "full"   # lite | full | ultra
language = "pt-BR"

[install]
cursor = true
copilot = true
antigravity = true

[mcp]
enabled = true
"#;

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct BrainforgeConfig {
    pub brainforge: BrainforgeMeta,
    pub install: InstallSection,
    pub mcp: McpSection,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct BrainforgeMeta {
    pub version: String,
    pub memory_dir: String,
    pub caveman_level: String,
    pub language: String,
}

impl Default for BrainforgeMeta {
    fn default() -> Self {
        Self {
            version: "1".into(),
            memory_dir: "brainforge/memory".into(),
            caveman_level: "full".into(),
            language: "pt-BR".into(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct InstallSection {
    pub cursor: bool,
    pub copilot: bool,
    pub antigravity: bool,
}

impl Default for InstallSection {
    fn default() -> Self {
        Self {
            cursor: true,
            copilot: true,
            antigravity: true,
        }
    }
}

impl InstallSection {
    pub fn enabled_adapters(&self) -> Vec<Adapter> {
        let mut out = Vec::with_capacity(3);
        if self.cursor {
            out.push(Adapter::Cursor);
        }
        if self.copilot {
            out.push(Adapter::Copilot);
        }
        if self.antigravity {
            out.push(Adapter::Antigravity);
        }
        out
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct McpSection {
    pub enabled: bool,
}

impl Default for McpSection {
    fn default() -> Self {
        Self { enabled: true }
    }
}

/// Load `brainforge.toml` from project root, or defaults if missing/invalid.
pub fn load_config(project_root: &Path) -> BrainforgeConfig {
    let path = project_root.join("brainforge.toml");
    if !path.is_file() {
        return BrainforgeConfig::default();
    }
    match std::fs::read_to_string(&path) {
        Ok(text) => match toml::from_str(&text) {
            Ok(cfg) => cfg,
            Err(e) => {
                eprintln!("warn: invalid brainforge.toml ({e}); using defaults");
                BrainforgeConfig::default()
            }
        },
        Err(e) => {
            eprintln!("warn: cannot read brainforge.toml ({e}); using defaults");
            BrainforgeConfig::default()
        }
    }
}

pub fn write_default_config_if_missing(project_root: &Path) -> Result<bool> {
    let path = project_root.join("brainforge.toml");
    if path.is_file() {
        return Ok(false);
    }
    std::fs::write(&path, DEFAULT_CONFIG_TEMPLATE)
        .with_context(|| format!("write {}", path.display()))?;
    Ok(true)
}

/// Expected canonical memory directory relative to project root.
pub fn expected_memory_dir() -> &'static str {
    "brainforge/memory"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_minimal_toml() {
        let cfg: BrainforgeConfig = toml::from_str(
            r#"
[brainforge]
caveman_level = "lite"

[install]
copilot = false
"#,
        )
        .unwrap();
        assert_eq!(cfg.brainforge.caveman_level, "lite");
        assert!(!cfg.install.copilot);
        assert!(cfg.install.cursor);
    }

    #[test]
    fn enabled_adapters_respects_flags() {
        let cfg: BrainforgeConfig = toml::from_str(
            r#"
[install]
cursor = true
copilot = false
antigravity = true
"#,
        )
        .unwrap();
        let adapters = cfg.install.enabled_adapters();
        assert_eq!(adapters.len(), 2);
        assert!(adapters.contains(&Adapter::Cursor));
        assert!(adapters.contains(&Adapter::Antigravity));
    }
}
