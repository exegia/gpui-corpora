#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn negative_origin_is_a_warning_not_error() {
    let cfg = AppConfig {
        window_bounds: Some(PersistedWindowBounds {
            x: -100.0,
            y: 200.0,
            width: 800.0,
            height: 600.0,
        }),
        ..AppConfig::default()
    };
    let lints = validate_config(&cfg);
    assert!(
        lints
            .iter()
            .any(|l| l.field == "window_bounds.x" && l.severity == LintSeverity::Warning)
    );
    assert!(!has_lint_for(&lints, "window_bounds.y"));
}
