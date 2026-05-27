use regex::Regex;

/// A potential secret detected in memory text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SecretFinding {
    pub kind: &'static str,
    pub line_hint: String,
}

/// Scan text for common secret patterns. Returns empty if clean.
pub fn scan_secrets(text: &str) -> Vec<SecretFinding> {
    let mut findings = Vec::new();
    let patterns: &[(&str, &str)] = &[
        ("aws_access_key", r"(?i)AKIA[0-9A-Z]{16}"),
        ("aws_secret", r"(?i)aws[_-]?secret[_-]?access[_-]?key\s*[:=]\s*\S+"),
        ("github_pat", r"ghp_[A-Za-z0-9_]{20,}"),
        ("github_oauth", r"gho_[A-Za-z0-9_]{20,}"),
        ("openai_key", r"sk-[A-Za-z0-9]{20,}"),
        (
            "generic_api_key",
            r"(?i)(api[_-]?key|apikey|secret[_-]?key)\s*[:=]\s*\S{8,}",
        ),
        ("bearer_token", r"(?i)bearer\s+[A-Za-z0-9._-]{20,}"),
        ("private_key", r"-----BEGIN (?:RSA |EC |OPENSSH )?PRIVATE KEY-----"),
        (
            "jwt",
            r"eyJ[A-Za-z0-9_-]{10,}\.[A-Za-z0-9_-]{10,}\.[A-Za-z0-9_-]{10,}",
        ),
    ];

    for (kind, pat) in patterns {
        let Ok(re) = Regex::new(pat) else {
            continue;
        };
        for (i, line) in text.lines().enumerate() {
            if re.is_match(line) {
                let preview: String = line.chars().take(80).collect();
                findings.push(SecretFinding {
                    kind,
                    line_hint: format!("line {}: {preview}...", i + 1),
                });
                break;
            }
        }
    }

    findings
}

pub fn scan_entries(entries: &[String]) -> Vec<SecretFinding> {
    let mut all = Vec::new();
    for (i, entry) in entries.iter().enumerate() {
        for mut f in scan_secrets(entry) {
            f.line_hint = format!("[entry {}] {}", i + 1, f.line_hint);
            all.push(f);
        }
    }
    all
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_github_pat() {
        let sample = "token ghp_abcdefghijklmnopqrstuvwxyz1234567890";
        let hits = scan_secrets(sample);
        assert!(hits.iter().any(|h| h.kind == "github_pat"));
    }

    #[test]
    fn clean_text_passes() {
        let sample = "Use brainforge sync for adapters";
        assert!(scan_secrets(sample).is_empty());
    }
}
