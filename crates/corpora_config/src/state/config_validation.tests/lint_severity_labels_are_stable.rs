#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn lint_severity_labels_are_stable() {
    assert_eq!(LintSeverity::Info.as_str(), "info");
    assert_eq!(LintSeverity::Warning.as_str(), "warning");
    assert_eq!(LintSeverity::Error.as_str(), "error");
    assert_eq!(format!("{}", LintSeverity::Error), "error");
}
