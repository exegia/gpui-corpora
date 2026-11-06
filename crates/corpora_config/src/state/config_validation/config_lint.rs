#[allow(unused_imports)]
use super::*;

#[derive(Clone, Debug)]
pub struct ConfigLint {
    /// Dotted path to the offending field (e.g. `theme`, `window_bounds.x`).
    pub field: String,
    /// Human-readable description of the issue and the likely fallback.
    pub message: String,
    pub severity: LintSeverity,
}

impl ConfigLint {
    pub(crate) fn new(
        field: impl Into<String>,
        severity: LintSeverity,
        message: impl Into<String>,
    ) -> Self {
        Self {
            field: field.into(),
            severity,
            message: message.into(),
        }
    }
}
