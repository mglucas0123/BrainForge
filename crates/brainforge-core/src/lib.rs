mod adapter;
mod config;
mod copy_util;
mod doctor;
mod embedded;
mod install;
mod recall;
mod security;
mod kit;
pub mod memory;
mod skill;
mod sync;

pub use adapter::Adapter;
pub use config::{BrainforgeConfig, load_config};
pub use doctor::{DoctorReport, DoctorStatus, run_doctor};
pub use embedded::{embedded_command_count, write_embedded_commands};
pub use install::{InstallOptions, InstallReport, format_mcp_config_json, run_install};
pub use recall::{
    TranscriptSession, cursor_project_slug, discover_transcripts_dir, extract_text_lines,
    list_sessions, search_transcripts,
};
pub use security::{SecretFinding, scan_entries, scan_secrets};
pub use kit::KitPaths;
pub use memory::{
    CompressResult, MemoryAuditReport, MemoryFile, MemoryStats, MemoryTarget, WriteSkipReason,
    audit_memory, compress_memory, read_memory, refresh_memory, sync_memory_to_cursor,
};
pub use skill::{
    CORE_SKILL_IDS, SkillInstallReport, SkillsCatalog, install_optional_skill, is_core_skill,
    list_optional_ids, load_catalog,
};
pub use sync::run_sync;

