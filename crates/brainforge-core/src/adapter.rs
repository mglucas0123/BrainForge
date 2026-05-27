use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Adapter {
    Cursor,
    Copilot,
    Antigravity,
}

impl Adapter {
    pub const ALL: [Adapter; 3] = [Adapter::Cursor, Adapter::Copilot, Adapter::Antigravity];

    pub fn label(self) -> &'static str {
        match self {
            Adapter::Cursor => "Cursor",
            Adapter::Copilot => "GitHub Copilot (VS Code)",
            Adapter::Antigravity => "Antigravity",
        }
    }

    pub fn detail(self) -> &'static str {
        match self {
            Adapter::Cursor => ".cursor/ — skills, /brainforge, rules",
            Adapter::Copilot => ".github/copilot-instructions.md",
            Adapter::Antigravity => ".agents/ — rules + workflows",
        }
    }
}

impl fmt::Display for Adapter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.label())
    }
}

impl FromStr for Adapter {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "cursor" => Ok(Adapter::Cursor),
            "copilot" => Ok(Adapter::Copilot),
            "antigravity" | "anti" | "agents" => Ok(Adapter::Antigravity),
            "all" => Err("use Adapter::ALL instead of parsing 'all'".into()),
            other => Err(format!("unknown adapter '{other}' (cursor | copilot | antigravity | all)")),
        }
    }
}
