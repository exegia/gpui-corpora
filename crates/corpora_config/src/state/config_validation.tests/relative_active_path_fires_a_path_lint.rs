#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn relative_active_path_fires_a_path_lint() {
    let cfg = AppConfig {
        active_path: "settings".to_string(),
        ..AppConfig::default()
    };
    let lints = validate_config(&cfg);
    assert!(lints.iter().any(|l| l.field == "active_path"
        && l.severity == LintSeverity::Warning
        && l.message.contains("absolute")));
}
