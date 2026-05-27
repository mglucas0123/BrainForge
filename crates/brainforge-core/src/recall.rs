use std::fs;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use anyhow::{Context, Result, bail};
use serde::Deserialize;

/// Encode a canonical path string the way Cursor names `~/.cursor/projects/<slug>/`.
pub fn slug_from_path_display(path: &Path) -> String {
    let mut s = path.display().to_string();
    s = s.replace(':', "").replace(['\\', '/'], "-");
    while s.contains("--") {
        s = s.replace("--", "-");
    }
    s.trim_matches('-').to_string()
}

/// Encode project path the way Cursor names `~/.cursor/projects/<slug>/`.
pub fn cursor_project_slug(project_root: &Path) -> Result<String> {
    let canonical = project_root
        .canonicalize()
        .with_context(|| format!("canonicalize {}", project_root.display()))?;
    Ok(slug_from_path_display(&canonical))
}

pub fn discover_transcripts_dir(project_root: &Path) -> Result<Option<PathBuf>> {
    if let Ok(env) = std::env::var("BRAINFORGE_TRANSCRIPTS") {
        let p = PathBuf::from(env);
        if p.is_dir() {
            return Ok(Some(p));
        }
    }

    let home = dirs_cursor_home()?;
    let projects = home.join("projects");
    if !projects.is_dir() {
        return Ok(None);
    }

    let slug = cursor_project_slug(project_root)?;
    let exact = projects.join(&slug).join("agent-transcripts");
    if exact.is_dir() {
        return Ok(Some(exact));
    }

    let mut candidates = Vec::new();
    let mut best: Option<(usize, PathBuf)> = None;

    for entry in fs::read_dir(&projects).with_context(|| format!("read {}", projects.display()))? {
        let entry = entry?;
        if !entry.file_type()?.is_dir() {
            continue;
        }
        let name = entry.file_name().to_string_lossy().to_string();
        let transcripts = entry.path().join("agent-transcripts");
        if !transcripts.is_dir() {
            continue;
        }
        candidates.push(transcripts.clone());
        if name.eq_ignore_ascii_case(&slug) {
            return Ok(Some(transcripts));
        }
        if slug.contains(&name) || name.contains(&slug) {
            let score = name.len();
            if best.as_ref().map(|(s, _)| score > *s).unwrap_or(true) {
                best = Some((score, transcripts));
            }
        }
    }

    if candidates.len() == 1 {
        return Ok(Some(candidates.into_iter().next().unwrap()));
    }

    Ok(best.map(|(_, p)| p))
}

fn dirs_cursor_home() -> Result<PathBuf> {
    if let Ok(p) = std::env::var("CURSOR_HOME") {
        return Ok(PathBuf::from(p));
    }
    let home = std::env::var("USERPROFILE")
        .or_else(|_| std::env::var("HOME"))
        .context("USERPROFILE or HOME not set")?;
    Ok(PathBuf::from(home).join(".cursor"))
}

#[derive(Debug, Clone)]
pub struct TranscriptSession {
    pub id: String,
    pub path: PathBuf,
    pub modified: Option<SystemTime>,
    pub line_count: usize,
}

pub fn list_sessions(transcripts_dir: &Path) -> Result<Vec<TranscriptSession>> {
    let mut sessions = Vec::new();
    for entry in fs::read_dir(transcripts_dir)
        .with_context(|| format!("read {}", transcripts_dir.display()))?
    {
        let entry = entry?;
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }
        let id = entry.file_name().to_string_lossy().to_string();
        let jsonl = path.join(format!("{id}.jsonl"));
        if !jsonl.is_file() {
            continue;
        }
        let line_count = BufReader::new(fs::File::open(&jsonl)?)
            .lines()
            .count();
        let modified = fs::metadata(&jsonl).ok().and_then(|m| m.modified().ok());
        sessions.push(TranscriptSession {
            id,
            path: jsonl,
            modified,
            line_count,
        });
    }
    sessions.sort_by_key(|b| std::cmp::Reverse(b.modified));
    Ok(sessions)
}

#[derive(Debug, Deserialize)]
struct TranscriptLine {
    role: Option<String>,
    message: Option<TranscriptMessage>,
}

#[derive(Debug, Deserialize)]
struct TranscriptMessage {
    content: Option<Vec<TranscriptBlock>>,
}

#[derive(Debug, Deserialize)]
struct TranscriptBlock {
    #[serde(rename = "type")]
    block_type: Option<String>,
    text: Option<String>,
}

pub fn extract_text_lines(jsonl_path: &Path, max_lines: usize) -> Result<Vec<(String, String)>> {
    let file = fs::File::open(jsonl_path)
        .with_context(|| format!("open {}", jsonl_path.display()))?;
    let reader = BufReader::new(file);
    let mut out = Vec::new();

    for line in reader.lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }
        let parsed: TranscriptLine = match serde_json::from_str(&line) {
            Ok(v) => v,
            Err(_) => continue,
        };
        let role = parsed.role.unwrap_or_else(|| "unknown".into());
        let Some(text) = parsed
            .message
            .and_then(|m| m.content)
            .and_then(|blocks| {
                blocks.into_iter().find_map(|b| {
                    if b.block_type.as_deref() == Some("text") {
                        b.text
                    } else {
                        None
                    }
                })
            })
        else {
            continue;
        };
        let preview: String = text.chars().take(400).collect();
        out.push((role, preview));
        if out.len() >= max_lines {
            break;
        }
    }
    Ok(out)
}

pub fn search_transcripts(
    transcripts_dir: &Path,
    query: &str,
    limit: usize,
) -> Result<Vec<(String, usize, String)>> {
    let q = query.to_lowercase();
    if q.is_empty() {
        bail!("search query cannot be empty");
    }
    let sessions = list_sessions(transcripts_dir)?;
    let mut hits = Vec::new();

    for session in sessions {
        let file = fs::File::open(&session.path)?;
        for (i, line) in BufReader::new(file).lines().enumerate() {
            let line = line?;
            if line.to_lowercase().contains(&q) {
                let preview: String = line.chars().take(120).collect();
                hits.push((session.id.clone(), i + 1, preview));
                if hits.len() >= limit {
                    return Ok(hits);
                }
            }
        }
    }
    Ok(hits)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slug_windows_style_path() {
        let slug = slug_from_path_display(Path::new(r"d:\Projetos\BrainForge"));
        assert!(slug.contains("Projetos"));
        assert!(!slug.contains(':'));
    }

    #[test]
    fn slug_from_existing_directory() {
        let dir = tempfile::tempdir().unwrap();
        let slug = cursor_project_slug(dir.path()).unwrap();
        assert!(!slug.is_empty());
        assert!(!slug.contains(':'));
    }
}
