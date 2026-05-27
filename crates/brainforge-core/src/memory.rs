use std::fmt;

use anyhow::{Context, Result};
use regex::Regex;

use crate::kit::KitPaths;

// ── Limits ──────────────────────────────────────────────────────────

/// Memory file targets with their char limits and thresholds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryTarget {
    ContextMd,
    UserMd,
}

impl MemoryTarget {
    pub const ALL: [MemoryTarget; 2] = [MemoryTarget::ContextMd, MemoryTarget::UserMd];

    pub fn filename(self) -> &'static str {
        match self {
            MemoryTarget::ContextMd => ".context.md",
            MemoryTarget::UserMd => ".user.md",
        }
    }

    pub fn char_limit(self) -> usize {
        match self {
            MemoryTarget::ContextMd => 2200,
            MemoryTarget::UserMd => 1375,
        }
    }

    pub fn line_threshold(self) -> usize {
        match self {
            MemoryTarget::ContextMd => 80,
            MemoryTarget::UserMd => 50,
        }
    }

    pub fn byte_threshold(self) -> usize {
        match self {
            MemoryTarget::ContextMd => 4096,
            MemoryTarget::UserMd => 2048,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            MemoryTarget::ContextMd => "context",
            MemoryTarget::UserMd => "user",
        }
    }

    /// Resolve path inside brainforge/memory/
    pub fn path(self, paths: &KitPaths) -> std::path::PathBuf {
        paths.memory().join(self.filename())
    }
}

impl fmt::Display for MemoryTarget {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.filename())
    }
}

// ── Parsed memory file ──────────────────────────────────────────────

/// A parsed BrainForge memory file.
#[derive(Debug, Clone)]
pub struct MemoryFile {
    /// e.g. `# Project Memory` or `# User Profile`
    pub title: String,
    /// Raw `**Capacity:**` line (will be recomputed)
    pub capacity_line: String,
    /// Content of each `§...§` block (without the `§` delimiters)
    pub entries: Vec<String>,
    /// Optional `## Reference` section content (preserved verbatim)
    pub reference: Option<String>,
    /// Char limit for this file (2200 or 1375)
    pub char_limit: usize,
    /// Line ending detected from the source file ("\r\n" or "\n")
    pub eol: String,
}

impl MemoryFile {
    /// Parse a memory file from its raw text.
    pub fn parse(text: &str, char_limit: usize) -> Result<Self> {
        // Detect line ending from source
        let eol = if text.contains("\r\n") {
            "\r\n".to_string()
        } else {
            "\n".to_string()
        };

        let lines: Vec<&str> = text.lines().collect();

        // Title: first `# ` line
        let title = lines
            .iter()
            .find(|l| l.starts_with("# "))
            .map(|l| l.to_string())
            .unwrap_or_else(|| "# Memory".to_string());

        // Capacity line
        let capacity_line = lines
            .iter()
            .find(|l| l.contains("**Capacity:**"))
            .map(|l| l.to_string())
            .unwrap_or_default();

        // Find ## Entries and ## Reference boundaries
        let entries_start = lines
            .iter()
            .position(|l| l.trim() == "## Entries")
            .unwrap_or(0);

        let reference_start = lines.iter().position(|l| l.trim() == "## Reference");

        // Extract entries region
        let entries_end = reference_start.unwrap_or(lines.len());
        let entries_region = &lines[entries_start..entries_end];
        let entries_text = entries_region.join("\n");

        // Parse § entries using regex
        let entries = parse_section_entries(&entries_text);

        // Extract reference section
        let reference = reference_start.map(|idx| {
            lines[idx..].join("\n")
        });

        Ok(Self {
            title,
            capacity_line,
            entries,
            reference,
            char_limit,
            eol,
        })
    }

    /// Total chars used by entries (including § delimiters).
    pub fn entry_chars(&self) -> usize {
        self.entries
            .iter()
            .map(|e| e.len() + 2) // +2 for §...§
            .sum()
    }

    /// Capacity percentage.
    pub fn capacity_pct(&self) -> usize {
        let used = self.entry_chars();
        if self.char_limit == 0 {
            return 0;
        }
        (used * 100) / self.char_limit
    }

    /// Recompute and update the capacity line.
    pub fn recompute_capacity(&mut self) {
        let used = self.entry_chars();
        let pct = self.capacity_pct();
        self.capacity_line = format!(
            "**Capacity:** {}% · {}/{} chars · ≥80% → consolidate before add",
            pct, used, self.char_limit
        );
    }

    /// True when the stored `**Capacity:**` line does not match live § char count.
    pub fn header_stale(&self) -> bool {
        !self.capacity_line.contains(&format!("{}%", self.capacity_pct()))
            || !self
                .capacity_line
                .contains(&format!("{}/{}", self.entry_chars(), self.char_limit))
    }

    /// Render the memory file back to markdown text, preserving original EOL.
    pub fn render(&self) -> String {
        let eol = &self.eol;
        let mut out = String::new();
        out.push_str(&self.title);
        out.push_str(eol);
        out.push_str(eol);
        out.push_str(&self.capacity_line);
        out.push_str(eol);
        out.push_str(eol);
        out.push_str("## Entries");
        out.push_str(eol);
        out.push_str(eol);

        for entry in &self.entries {
            out.push_str(&format!("§{}§", entry));
            out.push_str(eol);
        }

        if let Some(ref reference) = self.reference {
            out.push_str(eol);
            // Reference section already includes ## Reference heading
            for line in reference.lines() {
                out.push_str(line);
                out.push_str(eol);
            }
        }

        out
    }

    /// Deterministic compression: remove filler, merge duplicates, recompute capacity.
    pub fn compress(&mut self) {
        // Step 1: compress each entry individually
        self.entries = self
            .entries
            .iter()
            .map(|e| compress_entry(e))
            .filter(|e| !e.is_empty())
            .collect();

        // Step 2: merge entries with significant substring overlap
        self.entries = merge_duplicate_entries(&self.entries);

        // Step 3: recompute capacity
        self.recompute_capacity();
    }

    /// Validate against a pre-compress snapshot (`allow_merge` permits fewer § after merge).
    pub fn validate(&self, original: &MemoryFile, allow_merge: bool) -> Vec<String> {
        let mut issues = Vec::new();

        // Check capacity line present
        if !self.capacity_line.contains("**Capacity:**") {
            issues.push("Missing **Capacity:** line".to_string());
        }

        // Check capacity math consistency
        let used = self.entry_chars();
        let pct = self.capacity_pct();
        if !self.capacity_line.contains(&format!("{}%", pct))
            || !self.capacity_line.contains(&format!("{}/{}", used, self.char_limit))
        {
            issues.push(format!(
                "Capacity line inconsistent: expected {}% · {}/{}",
                pct, used, self.char_limit
            ));
        }

        // Check § entry count: compress should not silently drop entries
        // (merging is OK, but count should never exceed original)
        if self.entries.is_empty() && !original.entries.is_empty() {
            issues.push("All § entries were dropped".to_string());
        }

        if self.entries.len() < original.entries.len() && !allow_merge {
            issues.push(format!(
                "§ count {} → {} — re-run with --allow-merge to accept merge",
                original.entries.len(),
                self.entries.len()
            ));
        }

        // Build full text from both entries + reference for URL/path checking
        let orig_full = build_full_text(&original.entries, original.reference.as_deref());
        let new_full = build_full_text(&self.entries, self.reference.as_deref());

        // Check URLs/paths preserved (entries + reference)
        let orig_urls = extract_urls_and_paths(&orig_full);
        for url in &orig_urls {
            if !new_full.contains(url.as_str()) {
                issues.push(format!("Lost URL/path: {}", url));
            }
        }

        // Check fenced code blocks count (in reference section)
        let orig_fences = count_fences(original.reference.as_deref().unwrap_or(""));
        let new_fences = count_fences(self.reference.as_deref().unwrap_or(""));
        if orig_fences != new_fences {
            issues.push(format!(
                "Fenced code block count changed: {} → {}",
                orig_fences, new_fences
            ));
        }

        for (i, entry) in original.entries.iter().enumerate() {
            let anchors = extract_entry_anchors(entry);
            if anchors.is_empty() {
                continue;
            }
            let preserved = anchors.iter().all(|a| new_full.contains(a.as_str()));
            if !preserved {
                let preview: String = entry.chars().take(56).collect();
                issues.push(format!(
                    "§[{}] anchors lost (code/URL/path): {}…",
                    i + 1,
                    preview
                ));
            }
        }

        issues
    }

    /// Check if file exceeds auto-compress thresholds.
    pub fn exceeds_threshold(&self, target: MemoryTarget, raw_text: &str) -> bool {
        let line_count = raw_text.lines().count();
        let byte_count = raw_text.len();
        line_count > target.line_threshold() || byte_count > target.byte_threshold()
    }
}

// ── Entry parsing ───────────────────────────────────────────────────

fn parse_section_entries(text: &str) -> Vec<String> {
    let re = Regex::new(r"§([^§]+)§").expect("valid regex");
    re.captures_iter(text)
        .map(|c| c[1].trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

// ── Deterministic compression ───────────────────────────────────────

/// Remove filler words, articles, and hedging from a single § entry.
fn compress_entry(entry: &str) -> String {
    let mut text = entry.to_string();

    // Preserve inline code, URLs, paths, env vars by replacing temporarily
    let code_re = Regex::new(r"`[^`]+`").expect("valid regex");
    let mut preserved: Vec<String> = Vec::new();
    let mut idx = 0;
    text = code_re
        .replace_all(&text, |caps: &regex::Captures| {
            let placeholder = format!("\x00CODE{}\x00", idx);
            preserved.push(caps[0].to_string());
            idx += 1;
            placeholder
        })
        .to_string();

    // Remove common filler patterns (Portuguese and English)
    let filler_patterns = [
        // Portuguese articles and filler
        r"(?i)\bo que\b",
        r"(?i)\bno entanto\b",
        r"(?i)\bpor exemplo\b",
        r"(?i)\bbasicamente\b",
        r"(?i)\bgeralmente\b",
        r"(?i)\bnormalmente\b",
        r"(?i)\batualmente\b",
        r"(?i)\bessencialmente\b",
        r"(?i)\bpraticamente\b",
        r"(?i)\bsimplesmente\b",
        r"(?i)\brealmente\b",
        r"(?i)\beventualmente\b",
        r"(?i)\bobviamente\b",
        // English articles and filler
        r"(?i)\bbasically\b",
        r"(?i)\bgenerally\b",
        r"(?i)\bnormally\b",
        r"(?i)\bcurrently\b",
        r"(?i)\bessentially\b",
        r"(?i)\bpractically\b",
        r"(?i)\bsimply\b",
        r"(?i)\breally\b",
        r"(?i)\beventually\b",
        r"(?i)\bobviously\b",
        r"(?i)\bthe\s+",
        r"(?i)\ba\s+(?=[a-z])",
        r"(?i)\ban\s+(?=[a-z])",
    ];

    for pattern in &filler_patterns {
        if let Ok(re) = Regex::new(pattern) {
            text = re.replace_all(&text, "").to_string();
        }
    }

    // Collapse multiple spaces
    let space_re = Regex::new(r"  +").expect("valid regex");
    text = space_re.replace_all(&text, " ").to_string();

    // Restore preserved tokens
    for (i, original) in preserved.iter().enumerate() {
        let placeholder = format!("\x00CODE{}\x00", i);
        text = text.replace(&placeholder, original);
    }

    text.trim().to_string()
}

const MERGE_JACCARD_MIN: f64 = 0.85;
const MERGE_SUBSTRING_MIN: usize = 40;
const MERGE_SUBSTRING_JACCARD_FLOOR: f64 = 0.70;

/// Merge entries only with strong overlap (stopword-filtered Jaccard + long shared substring).
fn merge_duplicate_entries(entries: &[String]) -> Vec<String> {
    if entries.len() <= 1 {
        return entries.to_vec();
    }

    let mut result: Vec<String> = Vec::new();
    let mut merged: Vec<bool> = vec![false; entries.len()];

    for i in 0..entries.len() {
        if merged[i] {
            continue;
        }
        let mut current = entries[i].clone();

        for j in (i + 1)..entries.len() {
            if merged[j] {
                continue;
            }
            if should_merge_entries(&current, &entries[j]) {
                let unique_parts = find_unique_content(&entries[j], &current);
                if !unique_parts.is_empty() {
                    current = format!("{}; {}", current, unique_parts);
                }
                merged[j] = true;
            }
        }

        result.push(current);
    }

    result
}

fn should_merge_entries(a: &str, b: &str) -> bool {
    let jac = jaccard_significant_words(a, b);
    if jac >= MERGE_JACCARD_MIN {
        return true;
    }
    jac >= MERGE_SUBSTRING_JACCARD_FLOOR && has_shared_substring(a, b, MERGE_SUBSTRING_MIN)
}

fn normalize_token(word: &str) -> String {
    word.trim_matches(|c: char| !c.is_alphanumeric() && c != '/' && c != '\\' && c != '_' && c != '-' && c != '.')
        .to_lowercase()
}

fn is_merge_stopword(token: &str) -> bool {
    if token.len() < 3 {
        return true;
    }
    matches!(
        token,
        "brainforge"
            | "cursor"
            | "copilot"
            | "antigravity"
            | "github"
            | "agents"
            | "memory"
            | "project"
            | "skills"
            | "sync"
            | "the"
            | "and"
            | "for"
            | "com"
            | "para"
            | "uso"
            | "via"
            | "only"
            | "tool"
            | "tools"
            | "core"
            | "optional"
            | "install"
            | "slash"
            | "always"
            | "on"
            | "rust"
            | "cli"
            | "rtk"
            | "user"
            | "file"
            | "path"
    ) || token.starts_with(".cursor")
        || token.starts_with(".github")
        || token.starts_with(".agents")
}

fn significant_word_set(text: &str) -> std::collections::HashSet<String> {
    text.split_whitespace()
        .map(normalize_token)
        .filter(|w| !w.is_empty() && !is_merge_stopword(w))
        .collect()
}

/// Jaccard on tokens excluding kit stopwords.
fn jaccard_significant_words(a: &str, b: &str) -> f64 {
    let words_a = significant_word_set(a);
    let words_b = significant_word_set(b);

    if words_a.is_empty() || words_b.is_empty() {
        if a.trim().to_lowercase() == b.trim().to_lowercase() {
            return 1.0;
        }
        return 0.0;
    }

    let intersection = words_a.intersection(&words_b).count();
    let union = words_a.union(&words_b).count();

    if union == 0 {
        return 0.0;
    }

    intersection as f64 / union as f64
}

fn has_shared_substring(a: &str, b: &str, min_len: usize) -> bool {
    let (short, long) = if a.len() <= b.len() { (a, b) } else { (b, a) };
    if short.len() < min_len {
        return false;
    }
    let short_lower = short.to_lowercase();
    let long_lower = long.to_lowercase();
    for start in 0..=short_lower.len().saturating_sub(min_len) {
        if long_lower.contains(&short_lower[start..start + min_len]) {
            return true;
        }
    }
    false
}

/// Jaccard similarity on raw word sets (tests).
#[cfg(test)]
fn jaccard_word_similarity(a: &str, b: &str) -> f64 {
    let words_a: std::collections::HashSet<&str> = a.split_whitespace().collect();
    let words_b: std::collections::HashSet<&str> = b.split_whitespace().collect();

    if words_a.is_empty() && words_b.is_empty() {
        return 1.0;
    }

    let intersection = words_a.intersection(&words_b).count();
    let union = words_a.union(&words_b).count();

    if union == 0 {
        return 0.0;
    }

    intersection as f64 / union as f64
}

/// Find content in `source` not already present in `existing`.
fn find_unique_content(source: &str, existing: &str) -> String {
    let existing_normalized: std::collections::HashSet<String> = existing
        .split_whitespace()
        .map(normalize_token)
        .filter(|w| !w.is_empty())
        .collect();

    let mut unique = Vec::new();
    let mut seen_in_source = std::collections::HashSet::new();

    for w in source.split_whitespace() {
        let norm = normalize_token(w);
        if !norm.is_empty() && !existing_normalized.contains(&norm) && seen_in_source.insert(norm) {
            unique.push(w);
        }
    }
    unique.join(" ")
}

// ── Helpers ─────────────────────────────────────────────────────────

/// Build combined text from entries and optional reference for validation.
fn build_full_text(entries: &[String], reference: Option<&str>) -> String {
    let mut text = entries.join(" ");
    if let Some(r) = reference {
        text.push(' ');
        text.push_str(r);
    }
    text
}


fn extract_entry_anchors(entry: &str) -> Vec<String> {
    let mut anchors = extract_urls_and_paths(entry);
    let code_re = Regex::new(r"`([^`]+)`").expect("valid regex");
    for cap in code_re.captures_iter(entry) {
        anchors.push(cap[1].to_string());
    }
    for word in entry.split_whitespace() {
        if word.len() >= 20 {
            anchors.push(word.to_string());
        }
    }
    anchors
}

fn extract_urls_and_paths(text: &str) -> Vec<String> {
    let mut results = Vec::new();

    // URLs
    let url_re = Regex::new(r"https?://[^\s)\]]+").expect("valid regex");
    for m in url_re.find_iter(text) {
        results.push(m.as_str().to_string());
    }

    // File paths (Windows and Unix style)
    let path_re = Regex::new(r"(?:[A-Z]:\\|\.[\\/]|/)[^\s,;)]+").expect("valid regex");
    for m in path_re.find_iter(text) {
        results.push(m.as_str().to_string());
    }

    results
}

fn count_fences(text: &str) -> usize {
    text.lines().filter(|l| l.trim().starts_with("```")).count() / 2
}

// ── Public API ──────────────────────────────────────────────────────

/// Stats returned from reading a memory file.
#[derive(Debug)]
pub struct MemoryStats {
    pub target: MemoryTarget,
    pub entry_count: usize,
    pub char_used: usize,
    pub char_limit: usize,
    pub capacity_pct: usize,
    pub line_count: usize,
    pub byte_count: usize,
    pub exceeds_threshold: bool,
    pub has_reference: bool,
}

/// Read a memory file and return parsed stats.
pub fn read_memory(paths: &KitPaths, target: MemoryTarget) -> Result<(MemoryFile, MemoryStats)> {
    let filepath = target.path(paths);
    let text = std::fs::read_to_string(&filepath)
        .with_context(|| format!("read {}", filepath.display()))?;

    let mem = MemoryFile::parse(&text, target.char_limit())?;

    let stats = MemoryStats {
        target,
        entry_count: mem.entries.len(),
        char_used: mem.entry_chars(),
        char_limit: target.char_limit(),
        capacity_pct: mem.capacity_pct(),
        line_count: text.lines().count(),
        byte_count: text.len(),
        exceeds_threshold: mem.exceeds_threshold(target, &text),
        has_reference: mem.reference.is_some(),
    };

    Ok((mem, stats))
}

/// Why a memory write was skipped.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WriteSkipReason {
    DryRun,
    ValidationFailed,
    Unchanged,
}

/// Result of compress or refresh.
#[derive(Debug)]
pub struct CompressResult {
    pub target: MemoryTarget,
    pub chars_before: usize,
    pub chars_after: usize,
    pub entries_before: usize,
    pub entries_after: usize,
    pub validation_issues: Vec<String>,
    pub written: bool,
    pub skip_reason: Option<WriteSkipReason>,
}

fn decide_write(
    original_text: &str,
    original: &MemoryFile,
    updated: &MemoryFile,
    dry_run: bool,
    validation_issues: &[String],
) -> (bool, Option<WriteSkipReason>) {
    if dry_run {
        return (false, Some(WriteSkipReason::DryRun));
    }
    if !validation_issues.is_empty() {
        return (false, Some(WriteSkipReason::ValidationFailed));
    }

    let header_was_stale = original.header_stale();
    let chars_shrunk = updated.entry_chars() < original.entry_chars();
    let entries_shrunk = updated.entries.len() < original.entries.len();
    let content_changed = updated.render() != original_text;

    if header_was_stale || chars_shrunk || entries_shrunk || content_changed {
        (true, None)
    } else {
        (false, Some(WriteSkipReason::Unchanged))
    }
}

fn write_memory_file(filepath: &std::path::Path, mem: &MemoryFile) -> Result<()> {
    let rendered = mem.render();
    std::fs::write(filepath, &rendered)
        .with_context(|| format!("write {}", filepath.display()))?;
    Ok(())
}

/// Compress a memory file (local heuristic). Skips disk write if nothing changed.
pub fn compress_memory(
    paths: &KitPaths,
    target: MemoryTarget,
    dry_run: bool,
    allow_merge: bool,
) -> Result<CompressResult> {
    let filepath = target.path(paths);
    let text = std::fs::read_to_string(&filepath)
        .with_context(|| format!("read {}", filepath.display()))?;

    let original = MemoryFile::parse(&text, target.char_limit())?;
    let chars_before = original.entry_chars();
    let entries_before = original.entries.len();

    let mut compressed = original.clone();
    compressed.compress();

    let chars_after = compressed.entry_chars();
    let entries_after = compressed.entries.len();
    let validation_issues = compressed.validate(&original, allow_merge);

    let (should_write, skip_reason) =
        decide_write(&text, &original, &compressed, dry_run, &validation_issues);

    let written = if should_write {
        write_memory_file(&filepath, &compressed)?;
        true
    } else {
        false
    };

    Ok(CompressResult {
        target,
        chars_before,
        chars_after,
        entries_before,
        entries_after,
        validation_issues,
        written,
        skip_reason,
    })
}

/// Audit on-disk memory (format, header, mirror drift) without modifying files.
#[derive(Debug)]
pub struct MemoryAuditReport {
    pub target: MemoryTarget,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

impl MemoryAuditReport {
    pub fn ok(&self) -> bool {
        self.errors.is_empty()
    }
}

pub fn audit_memory(paths: &KitPaths, target: MemoryTarget) -> Result<MemoryAuditReport> {
    let mut errors = Vec::new();
    let mut warnings = Vec::new();

    let canonical = target.path(paths);
    if !canonical.is_file() {
        errors.push(format!("missing {}", canonical.display()));
        return Ok(MemoryAuditReport {
            target,
            errors,
            warnings,
        });
    }

    let text = std::fs::read_to_string(&canonical)
        .with_context(|| format!("read {}", canonical.display()))?;

    if text.trim().is_empty() {
        errors.push("file is empty".to_string());
        return Ok(MemoryAuditReport {
            target,
            errors,
            warnings,
        });
    }

    let mem = MemoryFile::parse(&text, target.char_limit())?;

    if mem.entries.is_empty() {
        errors.push("no § entries in ## Entries".to_string());
    }

    if !mem.capacity_line.contains("**Capacity:**") {
        errors.push("missing **Capacity:** header line".to_string());
    } else if mem.header_stale() {
        warnings.push("header stale — run `brainforge memory refresh`".to_string());
    } else if !mem
        .capacity_line
        .contains(&format!("{}/{}", mem.entry_chars(), mem.char_limit))
    {
        errors.push("capacity line does not match § char count".to_string());
    }

    let mirror = paths
        .project_root
        .join(".cursor")
        .join("project")
        .join(target.filename());
    if mirror.is_file() {
        let mirror_text = std::fs::read_to_string(&mirror).unwrap_or_default();
        if mirror_text != text {
            warnings.push(format!(
                "mirror drift: {} differs from {}",
                mirror.display(),
                canonical.display()
            ));
        }
    } else {
        warnings.push(format!(
            "mirror missing: {} — run `brainforge sync -a cursor`",
            mirror.display()
        ));
    }

    Ok(MemoryAuditReport {
        target,
        errors,
        warnings,
    })
}

/// Recompute `**Capacity:**` only — no filler removal or § merge.
pub fn refresh_memory(
    paths: &KitPaths,
    target: MemoryTarget,
    dry_run: bool,
) -> Result<CompressResult> {
    let filepath = target.path(paths);
    let text = std::fs::read_to_string(&filepath)
        .with_context(|| format!("read {}", filepath.display()))?;

    let original = MemoryFile::parse(&text, target.char_limit())?;
    let chars_before = original.entry_chars();
    let entries_before = original.entries.len();

    let mut refreshed = original.clone();
    refreshed.recompute_capacity();

    let chars_after = refreshed.entry_chars();
    let entries_after = refreshed.entries.len();
    let validation_issues = refreshed.validate(&original, true);

    let (should_write, skip_reason) =
        decide_write(&text, &original, &refreshed, dry_run, &validation_issues);

    let written = if should_write {
        write_memory_file(&filepath, &refreshed)?;
        true
    } else {
        false
    };

    Ok(CompressResult {
        target,
        chars_before,
        chars_after,
        entries_before,
        entries_after,
        validation_issues,
        written,
        skip_reason,
    })
}

/// Post-compress: sync memory to .cursor/project/ if requested.
pub fn sync_memory_to_cursor(paths: &KitPaths) -> Result<()> {
    let cursor_project = paths.project_root.join(".cursor").join("project");
    crate::copy_util::ensure_dir(&cursor_project)?;

    for target in MemoryTarget::ALL {
        let src = target.path(paths);
        if src.is_file() {
            let dst = cursor_project.join(target.filename());
            crate::copy_util::copy_file(&src, &dst)?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_CONTEXT: &str = "\
# Project Memory

**Capacity:** 43% · 940/2200 chars · ≥80% → consolidate before add

## Entries

§BrainForge = portable kit; source `brainforge/`; Rust CLI `brainforge sync|doctor` → `.cursor`, `.github`, `.agents`§
§Memory canonical `brainforge/memory/`; mirror `.cursor/project/`; learnings `.cursor/learnings.md`§

## Reference

```powershell
.\\brainforge\\tools\\rtk\\rtk.exe gain
```
";

    #[test]
    fn parse_entries() {
        let mem = MemoryFile::parse(SAMPLE_CONTEXT, 2200).unwrap();
        assert_eq!(mem.entries.len(), 2);
        assert!(mem.entries[0].contains("BrainForge"));
        assert!(mem.entries[1].contains("Memory canonical"));
    }

    #[test]
    fn capacity_recompute() {
        let mut mem = MemoryFile::parse(SAMPLE_CONTEXT, 2200).unwrap();
        mem.recompute_capacity();
        assert!(mem.capacity_line.contains("**Capacity:**"));
        assert!(mem.capacity_line.contains("/2200"));
    }

    #[test]
    fn render_roundtrip() {
        let mem = MemoryFile::parse(SAMPLE_CONTEXT, 2200).unwrap();
        let rendered = mem.render();
        assert!(rendered.contains("# Project Memory"));
        assert!(rendered.contains("## Entries"));
        assert!(rendered.contains("§"));
        assert!(rendered.contains("## Reference"));
        assert!(rendered.contains("```powershell"));
    }

    #[test]
    fn compress_removes_filler() {
        let entry = "basically the project currently uses normally this approach";
        let compressed = compress_entry(entry);
        assert!(!compressed.contains("basically"));
        assert!(!compressed.contains("currently"));
        assert!(!compressed.contains("normally"));
    }

    #[test]
    fn compress_preserves_code() {
        let entry = "use `brainforge sync` to basically deploy";
        let compressed = compress_entry(entry);
        assert!(compressed.contains("`brainforge sync`"));
    }

    #[test]
    fn validate_catches_missing_url() {
        let original = MemoryFile {
            title: "# Test".to_string(),
            capacity_line: "**Capacity:** 10% · 50/500 chars · ≥80% → consolidate before add"
                .to_string(),
            entries: vec!["see https://example.com/doc for info".to_string()],
            reference: None,
            char_limit: 500,
            eol: "\n".to_string(),
        };
        let mut modified = original.clone();
        modified.entries = vec!["see docs for info".to_string()];
        modified.recompute_capacity();
        let issues = modified.validate(&original, false);
        assert!(issues.iter().any(|i| i.contains("https://example.com")));
    }

    #[test]
    fn merge_skips_kit_stopword_only_overlap() {
        let a = "BrainForge cursor sync doctor paths kit";
        let b = "BrainForge cursor optional skills catalog json";
        assert!(!should_merge_entries(a, b));
    }

    #[test]
    fn shared_substring_detects_long_overlap() {
        let a = "alpha beta gamma delta epsilon zeta eta theta kappa lambda mu nu xi";
        let b = format!("{a} omicron pi rho sigma tau");
        assert!(has_shared_substring(a, &b, MERGE_SUBSTRING_MIN));
    }

    #[test]
    fn merge_allows_high_significant_overlap() {
        let a = "decision alpha beta gamma delta epsilon zeta eta theta kappa record";
        let b = "decision alpha beta gamma delta epsilon zeta eta theta kappa record extended";
        assert!(jaccard_significant_words(a, b) >= MERGE_JACCARD_MIN);
        assert!(should_merge_entries(a, b));
    }

    #[test]
    fn validate_blocks_merge_without_flag() {
        let original = MemoryFile {
            title: "# Test".to_string(),
            capacity_line: "**Capacity:** 20% · 100/500 chars · ≥80% → consolidate before add"
                .to_string(),
            entries: vec!["one fact here".to_string(), "other fact there".to_string()],
            reference: None,
            char_limit: 500,
            eol: "\n".to_string(),
        };
        let mut merged = original.clone();
        merged.entries = vec!["one fact here; other fact there".to_string()];
        merged.recompute_capacity();
        let strict = merged.validate(&original, false);
        assert!(strict.iter().any(|i| i.contains("§ count")));
        let loose = merged.validate(&original, true);
        assert!(!loose.iter().any(|i| i.contains("§ count")));
    }

    #[test]
    fn jaccard_similarity_identical() {
        assert!((jaccard_word_similarity("hello world", "hello world") - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn jaccard_similarity_different() {
        assert!(jaccard_word_similarity("hello world", "foo bar") < 0.1);
    }

    #[test]
    fn header_stale_detects_drift() {
        let mem = MemoryFile::parse(SAMPLE_CONTEXT, 2200).unwrap();
        assert!(mem.header_stale());
    }

    #[test]
    fn header_ok_after_recompute() {
        let mut mem = MemoryFile::parse(SAMPLE_CONTEXT, 2200).unwrap();
        mem.recompute_capacity();
        assert!(!mem.header_stale());
    }

    #[test]
    fn decide_write_after_stale_header_needs_write() {
        let mem = MemoryFile::parse(SAMPLE_CONTEXT, 2200).unwrap();
        assert!(mem.header_stale());
        let mut refreshed = mem.clone();
        refreshed.recompute_capacity();
        let (write, reason) = decide_write(SAMPLE_CONTEXT, &mem, &refreshed, false, &[]);
        assert!(write);
        assert!(reason.is_none());
    }

    #[test]
    fn decide_write_skips_when_identical() {
        let mut mem = MemoryFile::parse(SAMPLE_CONTEXT, 2200).unwrap();
        mem.recompute_capacity();
        let rendered = mem.render();
        let (write, reason) = decide_write(&rendered, &mem, &mem, false, &[]);
        assert!(!write);
        assert_eq!(reason, Some(WriteSkipReason::Unchanged));
    }

    #[test]
    fn compress_removes_filler_case_insensitive() {
        let entry = "Basically The project Currently uses Normally this approach";
        let compressed = compress_entry(entry);
        assert!(!compressed.to_lowercase().contains("basically"));
        assert!(!compressed.to_lowercase().contains("the"));
        assert!(!compressed.to_lowercase().contains("currently"));
        assert!(!compressed.to_lowercase().contains("normally"));
    }

    #[test]
    fn merge_skips_different_stopword_only_entries() {
        let a = "BrainForge cursor sync";
        let b = "Antigravity agents";
        assert!(!should_merge_entries(a, b));
    }

    #[test]
    fn find_unique_content_avoids_semicolon_duplicates() {
        let existing = "hello; world";
        let source = "hello";
        let unique = find_unique_content(source, existing);
        assert!(unique.is_empty());
    }
}

