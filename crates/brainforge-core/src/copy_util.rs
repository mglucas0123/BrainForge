use std::fs;
use std::path::Path;

use anyhow::{Context, Result};
use fs_extra::dir::CopyOptions;
use walkdir::WalkDir;

pub fn ensure_dir(path: &Path) -> Result<()> {
    if !path.exists() {
        fs::create_dir_all(path).with_context(|| format!("create dir {}", path.display()))?;
    }
    Ok(())
}

pub fn copy_file(src: &Path, dst: &Path) -> Result<()> {
    if let Some(parent) = dst.parent() {
        ensure_dir(parent)?;
    }
    fs::copy(src, dst).with_context(|| format!("copy {} -> {}", src.display(), dst.display()))?;
    Ok(())
}

pub fn copy_tree(src: &Path, dst: &Path) -> Result<()> {
    if !src.exists() {
        return Ok(());
    }
    ensure_dir(dst)?;
    let options = CopyOptions {
        content_only: true,
        overwrite: true,
        ..Default::default()
    };
    fs_extra::dir::copy(src, dst, &options)
        .with_context(|| format!("copy tree {} -> {}", src.display(), dst.display()))?;
    Ok(())
}

/// Replace destination children then copy each top-level entry from src.
pub fn mirror_dir_contents(src: &Path, dst: &Path) -> Result<()> {
    if !src.is_dir() {
        return Ok(());
    }
    ensure_dir(dst)?;
    for entry in fs::read_dir(src).with_context(|| format!("read dir {}", src.display()))? {
        let entry = entry?;
        let name = entry.file_name();
        let from = entry.path();
        let to = dst.join(&name);
        if from.is_dir() {
            if to.exists() {
                fs::remove_dir_all(&to).ok();
            }
            copy_tree(&from, &to)?;
        } else {
            copy_file(&from, &to)?;
        }
    }
    Ok(())
}

pub fn clear_dir_children(dir: &Path) -> Result<()> {
    if !dir.is_dir() {
        return Ok(());
    }
    for entry in fs::read_dir(dir).with_context(|| format!("read dir {}", dir.display()))? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            fs::remove_dir_all(&path).with_context(|| format!("remove {}", path.display()))?;
        } else {
            fs::remove_file(&path).with_context(|| format!("remove {}", path.display()))?;
        }
    }
    Ok(())
}

pub fn file_nonempty(path: &Path) -> bool {
    path.is_file() && fs::metadata(path).map(|m| m.len() > 0).unwrap_or(false)
}

pub fn read_to_string_lossy(path: &Path) -> Option<String> {
    fs::read_to_string(path).ok()
}

pub fn dir_has_files_with_ext(dir: &Path, ext: &str) -> bool {
    if !dir.is_dir() {
        return false;
    }
    WalkDir::new(dir)
        .max_depth(3)
        .into_iter()
        .filter_map(|e| e.ok())
        .any(|e| e.path().extension().is_some_and(|x| x == ext))
}
