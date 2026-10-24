#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn negative_height_fires_an_error_lint() {
    let cfg = AppConfig {
        window_bounds: Some(PersistedWindowBounds {
            x: 10.0,
            y: 10.0,
            width: 400.0,
            height: -1.0,
        }),
        ..AppConfig::default()
    };
    let lints = validate_config(&cfg);
    assert!(
        lints
            .iter()
            .any(|l| l.field == "window_bounds.height" && l.severity == LintSeverity::Error)
    );
}
