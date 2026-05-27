/// How adapter folders relate to the canonical `.brainforge/` kit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SyncMode {
    /// `.cursor/` / `.agents/` = thin bridges only; kit lives in `.brainforge/`.
    #[default]
    Thin,
    /// Full copy into `.cursor/` (legacy; large tree).
    Mirror,
}

impl SyncMode {
    pub fn parse(s: &str) -> Self {
        match s.trim().to_lowercase().as_str() {
            "mirror" | "full" => SyncMode::Mirror,
            _ => SyncMode::Thin,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            SyncMode::Thin => "thin",
            SyncMode::Mirror => "mirror",
        }
    }
}
