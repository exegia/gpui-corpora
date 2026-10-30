#[allow(unused_imports)]
use super::*;

/// `active_path` must be an absolute router path; route existence is the
/// router's call (unknown paths land on the not-found page).
pub(crate) fn validate_active_path(path: &str) -> Vec<ConfigLint> {
    if !path.starts_with('/') {
        return vec![ConfigLint::new(
            "active_path",
            LintSeverity::Warning,
            "active_path is not an absolute route path; the home route will be used",
        )];
    }
    Vec::new()
}
