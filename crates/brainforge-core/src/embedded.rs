use std::path::Path;

use anyhow::{Context, Result};
use include_dir::{Dir, include_dir};

use crate::copy_util::ensure_dir;

static EMBEDDED_COMMANDS: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/../../brainforge/core/commands");

/// Write slash commands from compile-time embed (kit absent or `--embed-commands`).
pub fn write_embedded_commands(dest: &Path) -> Result<usize> {
    ensure_dir(dest)?;
    let mut count = 0usize;
    for file in EMBEDDED_COMMANDS.files() {
        let Some(name) = file.path().file_name() else {
            continue;
        };
        let out = dest.join(name);
        let data = file.contents();
        if let Some(parent) = out.parent() {
            ensure_dir(parent)?;
        }
        std::fs::write(&out, data)
            .with_context(|| format!("write {}", out.display()))?;
        count += 1;
    }
    Ok(count)
}

pub fn embedded_command_count() -> usize {
    EMBEDDED_COMMANDS.files().count()
}
