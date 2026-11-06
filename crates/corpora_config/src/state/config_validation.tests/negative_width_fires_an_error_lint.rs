#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn negative_width_fires_an_error_lint() {
    let cfg = AppConfig {
        window_bounds: Some(PersistedWindowBounds {
            x: 10.0,
            y: 10.0,
            width: -50.0,
            height: 400.0,
        }),
        ..AppConfig::default()
    };
    let lints = validate_config(&cfg);
    assert!(
        lints
            .iter()
            .any(|l| l.field == "window_bounds.width" && l.severity == LintSeverity::Error)
    );
}
