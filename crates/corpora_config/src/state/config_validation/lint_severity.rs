#[allow(unused_imports)]
use super::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LintSeverity {
    /// Informational note; no action required (e.g. an unusual-but-legal value).
    Info,
    /// Something looks off and probably warrants attention, but the app will
    /// still run correctly (e.g. a window position that may be off-screen).
    Warning,
    /// A value the app cannot interpret; it will fall back to a default
    /// (e.g. an unknown locale or update channel).
    Error,
}

impl LintSeverity {
    pub fn as_str(self) -> &'static str {
        match self {
            LintSeverity::Info => "info",
            LintSeverity::Warning => "warning",
            LintSeverity::Error => "error",
        }
    }
}

impl std::fmt::Display for LintSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
