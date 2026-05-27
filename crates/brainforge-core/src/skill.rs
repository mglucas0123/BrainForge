use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};

use crate::copy_util::{copy_tree, ensure_dir};
use crate::kit::KitPaths;

/// Core skills — never install/overwrite from optional catalog.
pub const CORE_SKILL_IDS: &[&str] = &[
    "caveman",
    "cavemem",
    "cavecrew",
    "compress-context",
    "find-skills",
    "learning-loop",
    "skill-authoring",
    "curator",
    "install-skill",
    "brainforge-doctor",
    "refactor-master",
    "clean-coder",
    "bug-hunter",
    "systematic-debugging",
    "architect",
    "task-master",
];

pub fn is_core_skill(id: &str) -> bool {
    CORE_SKILL_IDS
        .iter()
        .any(|core| core.eq_ignore_ascii_case(id))
}

#[derive(Debug, Clone, Deserialize)]
pub struct SkillsCatalog {
    pub version: u32,
    pub optional_skills: Vec<OptionalSkillEntry>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct OptionalSkillEntry {
    pub id: String,
    pub name: String,
    pub description: String,
    pub source_path: String,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub effort: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstalledSkills {
    pub version: u32,
    #[serde(default)]
    pub installed: Vec<String>,
}

impl Default for InstalledSkills {
    fn default() -> Self {
        Self {
            version: 1,
            installed: vec![],
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct SkillInstallReport {
    pub id: String,
    pub source: PathBuf,
    pub target: PathBuf,
    pub already_installed: bool,
}

pub fn catalog_path(paths: &KitPaths) -> PathBuf {
    let project = paths.project_root.join(".cursor").join("skills-catalog.json");
    if project.is_file() {
        return project;
    }
    paths.core().join("skills-catalog.json")
}

pub fn installed_path(paths: &KitPaths) -> PathBuf {
    paths
        .project_root
        .join(".cursor")
        .join("installed-skills.json")
}

pub fn load_catalog(paths: &KitPaths) -> Result<SkillsCatalog> {
    let path = catalog_path(paths);
    let text = fs::read_to_string(&path)
        .with_context(|| format!("read catalog {}", path.display()))?;
    serde_json::from_str(&text).with_context(|| format!("parse catalog {}", path.display()))
}

pub fn load_installed(paths: &KitPaths) -> Result<InstalledSkills> {
    let path = installed_path(paths);
    if !path.is_file() {
        return Ok(InstalledSkills::default());
    }
    let text = fs::read_to_string(&path)
        .with_context(|| format!("read {}", path.display()))?;
    serde_json::from_str(&text).with_context(|| format!("parse {}", path.display()))
}

pub fn save_installed(paths: &KitPaths, data: &InstalledSkills) -> Result<()> {
    let path = installed_path(paths);
    ensure_dir(path.parent().unwrap())?;
    let text = serde_json::to_string_pretty(data).context("serialize installed-skills")?;
    fs::write(&path, text).with_context(|| format!("write {}", path.display()))
}

pub fn list_optional_ids(paths: &KitPaths) -> Result<Vec<String>> {
    let catalog = load_catalog(paths)?;
    Ok(catalog
        .optional_skills
        .iter()
        .map(|e| e.id.clone())
        .collect())
}

pub fn install_optional_skill(
    paths: &KitPaths,
    id: &str,
    force: bool,
) -> Result<SkillInstallReport> {
    let id = id.trim();
    if id.is_empty() {
        bail!("skill id cannot be empty");
    }
    if is_core_skill(id) {
        bail!("'{id}' is a core skill — cannot install from optional catalog");
    }

    let catalog = load_catalog(paths)?;
    let entry = catalog
        .optional_skills
        .iter()
        .find(|e| e.id.eq_ignore_ascii_case(id))
        .with_context(|| {
            let available = catalog
                .optional_skills
                .iter()
                .map(|e| e.id.as_str())
                .collect::<Vec<_>>()
                .join(", ");
            format!(
                "skill '{id}' not in catalog. Available: {}",
                if available.is_empty() {
                    "(none)".into()
                } else {
                    available
                }
            )
        })?;

    let cursor = paths.project_root.join(".cursor");
    let source = cursor.join(&entry.source_path);
    if !source.is_dir() {
        bail!(
            "optional source missing at {} — run `brainforge sync` first",
            source.display()
        );
    }

    let skill_md = source.join("SKILL.md");
    if !skill_md.is_file() {
        bail!("missing SKILL.md in {}", source.display());
    }

    let target = cursor.join("skills").join(&entry.id);
    if target.exists() {
        if !force {
            bail!(
                "{} already exists — use --force to replace",
                target.display()
            );
        }
        fs::remove_dir_all(&target)
            .with_context(|| format!("remove {}", target.display()))?;
    }

    ensure_dir(target.parent().unwrap())?;
    copy_tree(&source, &target)?;

    let mut installed = load_installed(paths)?;
    if !installed
        .installed
        .iter()
        .any(|x| x.eq_ignore_ascii_case(&entry.id))
    {
        installed.installed.push(entry.id.clone());
        installed.installed.sort();
        save_installed(paths, &installed)?;
    }

    register_usage(paths, &entry.id)?;

    Ok(SkillInstallReport {
        id: entry.id.clone(),
        source,
        target,
        already_installed: false,
    })
}

fn register_usage(paths: &KitPaths, id: &str) -> Result<()> {
    let usage_path = paths.project_root.join(".cursor/skills/.usage.json");
    ensure_dir(usage_path.parent().unwrap())?;

    let mut root: UsageRoot = if usage_path.is_file() {
        let text = fs::read_to_string(&usage_path)?;
        serde_json::from_str(&text).unwrap_or_default()
    } else {
        UsageRoot::default()
    };

    root.skills
        .entry(id.to_string())
        .or_insert_with(|| UsageEntry {
            created_by: "brainforge-cli".into(),
            use_count: 0,
            state: "active".into(),
            pinned: false,
        });

    let text = serde_json::to_string_pretty(&root).context("serialize .usage.json")?;
    fs::write(&usage_path, text).with_context(|| format!("write {}", usage_path.display()))?;
    Ok(())
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct UsageRoot {
    #[serde(default)]
    skills: HashMap<String, UsageEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
struct UsageEntry {
    created_by: String,
    use_count: u32,
    state: String,
    pinned: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn blocks_core_skill_ids() {
        assert!(is_core_skill("caveman"));
        assert!(!is_core_skill("writing-plans"));
    }

    #[test]
    fn parse_catalog_json() {
        let json = r#"{"version":1,"optional_skills":[{"id":"x","name":"x","description":"d","source_path":"skills-optional/x"}]}"#;
        let c: SkillsCatalog = serde_json::from_str(json).unwrap();
        assert_eq!(c.optional_skills[0].id, "x");
    }
}
