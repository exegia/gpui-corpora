#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn tiny_window_fires_a_width_and_height_lint() {
    let cfg = AppConfig {
        window_bounds: Some(PersistedWindowBounds {
            x: 0.0,
            y: 0.0,
            width: 10.0,
            height: 10.0,
        }),
        ..AppConfig::default()
    };
    let lints = validate_config(&cfg);
    assert!(has_lint_for(&lints, "window_bounds.width"));
    assert!(has_lint_for(&lints, "window_bounds.height"));
}
