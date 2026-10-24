#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn nan_dimension_fires_an_error_lint() {
    let cfg = AppConfig {
        window_bounds: Some(PersistedWindowBounds {
            x: 0.0,
            y: 0.0,
            width: f32::NAN,
            height: 600.0,
        }),
        ..AppConfig::default()
    };
    let lints = validate_config(&cfg);
    assert!(lints.iter().any(|l| l.field == "window_bounds.width"
        && l.severity == LintSeverity::Error
        && l.message.contains("NaN")));
}
