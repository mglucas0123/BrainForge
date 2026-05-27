use std::path::Path;

use crate::adapter::Adapter;
use crate::copy_util::{dir_has_files_with_ext, file_nonempty, read_to_string_lossy};
use crate::kit::KitPaths;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DoctorStatus {
    Pass,
    Warn,
    Fail,
}

#[derive(Debug, Clone)]
pub struct DoctorCheck {
    pub name: String,
    pub status: DoctorStatus,
    pub detail: String,
}

#[derive(Debug, Clone)]
pub struct DoctorReport {
    pub checks: Vec<DoctorCheck>,
}

impl DoctorReport {
    pub fn has_fail(&self) -> bool {
        self.checks
            .iter()
            .any(|c| c.status == DoctorStatus::Fail)
    }
}

pub fn run_doctor(paths: &KitPaths) -> Result<DoctorReport, anyhow::Error> {
    let mut checks = vec![
        check_rtk(paths),
        check_memory(
            "brainforge/memory/.context.md",
            &paths.memory().join(".context.md"),
        ),
        check_memory(
            "brainforge/memory/.user.md",
            &paths.memory().join(".user.md"),
        ),
        check_mirror_memory(
            ".cursor/project mirror",
            &paths.project_root.join(".cursor/project/.context.md"),
            &paths.memory().join(".context.md"),
        ),
        check_legacy_rtk(&paths.project_root),
        check_kit_marker(paths),
        check_dir(
            "core/skills",
            &paths.core().join("skills"),
            |p| dir_has_files_with_ext(p, "md"),
        ),
        check_file(
            "cursor rule",
            &paths.adapters().join("cursor/rules/cavecrew-default.mdc"),
        ),
        check_file(
            "BRAINFORGE routine",
            &paths.core().join("BRAINFORGE.md"),
        ),
    ];

    for adapter in Adapter::ALL {
        checks.push(check_adapter_outputs(paths, adapter));
    }

    Ok(DoctorReport { checks })
}

fn check_rtk(paths: &KitPaths) -> DoctorCheck {
    let p = paths.rtk_exe();
    if p.is_file() {
        DoctorCheck {
            name: "RTK".into(),
            status: DoctorStatus::Pass,
            detail: format!("{}", p.display()),
        }
    } else {
        DoctorCheck {
            name: "RTK".into(),
            status: DoctorStatus::Warn,
            detail: format!(
                "missing {} — run brainforge/tools/rtk/install-rtk-local.ps1 -Force",
                p.display()
            ),
        }
    }
}

fn check_memory(name: &str, path: &Path) -> DoctorCheck {
    if !file_nonempty(path) {
        return DoctorCheck {
            name: name.into(),
            status: DoctorStatus::Warn,
            detail: "missing or empty — create baseline § entries".into(),
        };
    }
    let text = read_to_string_lossy(path).unwrap_or_default();
    let ok_format = text.contains("**Capacity:**") && text.contains('§');
    DoctorCheck {
        name: name.into(),
        status: if ok_format {
            DoctorStatus::Pass
        } else {
            DoctorStatus::Warn
        },
        detail: if ok_format {
            "format OK".into()
        } else {
            "missing **Capacity:** or § entries".into()
        },
    }
}

fn check_mirror_memory(name: &str, mirror: &Path, canonical: &Path) -> DoctorCheck {
    if !mirror.exists() {
        return DoctorCheck {
            name: name.into(),
            status: DoctorStatus::Warn,
            detail: "mirror missing — run `brainforge sync -a cursor`".into(),
        };
    }
    let same = fs_same_content(mirror, canonical);
    DoctorCheck {
        name: name.into(),
        status: if same {
            DoctorStatus::Pass
        } else {
            DoctorStatus::Warn
        },
        detail: if same {
            "in sync with canonical".into()
        } else {
            "drift from brainforge/memory — re-sync".into()
        },
    }
}

fn fs_same_content(a: &Path, b: &Path) -> bool {
    match (std::fs::read(a), std::fs::read(b)) {
        (Ok(x), Ok(y)) => x == y,
        _ => false,
    }
}

fn check_legacy_rtk(project_root: &Path) -> DoctorCheck {
    let legacy = project_root.join(".cursor/tools/rtk");
    if legacy.exists() {
        DoctorCheck {
            name: "legacy RTK path".into(),
            status: DoctorStatus::Warn,
            detail: format!(
                "{} exists — remove; RTK lives only under brainforge/tools/rtk/",
                legacy.display()
            ),
        }
    } else {
        DoctorCheck {
            name: "legacy RTK path".into(),
            status: DoctorStatus::Pass,
            detail: "no .cursor/tools/rtk duplicate".into(),
        }
    }
}

fn check_kit_marker(paths: &KitPaths) -> DoctorCheck {
    let m = paths.core().join("BRAINFORGE.md");
    DoctorCheck {
        name: "kit root".into(),
        status: if m.is_file() {
            DoctorStatus::Pass
        } else {
            DoctorStatus::Fail
        },
        detail: paths.kit_root.display().to_string(),
    }
}

fn check_dir(name: &str, path: &Path, ok: fn(&Path) -> bool) -> DoctorCheck {
    DoctorCheck {
        name: name.into(),
        status: if ok(path) {
            DoctorStatus::Pass
        } else {
            DoctorStatus::Warn
        },
        detail: path.display().to_string(),
    }
}

fn check_file(name: &str, path: &Path) -> DoctorCheck {
    DoctorCheck {
        name: name.into(),
        status: if path.is_file() {
            DoctorStatus::Pass
        } else {
            DoctorStatus::Fail
        },
        detail: path.display().to_string(),
    }
}

fn check_adapter_outputs(paths: &KitPaths, adapter: Adapter) -> DoctorCheck {
    let (name, path) = match adapter {
        Adapter::Cursor => (
            "cursor output",
            paths.project_root.join(".cursor/commands/brainforge.md"),
        ),
        Adapter::Copilot => (
            "copilot output",
            paths
                .project_root
                .join(".github/copilot-instructions.md"),
        ),
        Adapter::Antigravity => (
            "antigravity output",
            paths
                .project_root
                .join(".agents/workflows/brainforge.md"),
        ),
    };
    DoctorCheck {
        name: name.into(),
        status: if path.is_file() {
            DoctorStatus::Pass
        } else {
            DoctorStatus::Warn
        },
        detail: if path.is_file() {
            "present".into()
        } else {
            format!("missing {} — run sync", path.display())
        },
    }
}
