#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn huge_window_fires_a_lint() {
    let cfg = AppConfig {
        window_bounds: Some(PersistedWindowBounds {
            x: 0.0,
            y: 0.0,
            width: 50_000.0,
            height: 600.0,
        }),
        ..AppConfig::default()
    };
    let lints = validate_config(&cfg);
    assert!(has_lint_for(&lints, "window_bounds.width"));
}
