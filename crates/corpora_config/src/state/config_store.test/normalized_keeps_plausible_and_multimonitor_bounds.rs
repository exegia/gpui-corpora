#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn normalized_keeps_plausible_and_multimonitor_bounds() {
    // A negative origin is legal (left-of-primary monitor), and a window
    // wider than the 8192 lint ceiling is still a real window worth keeping.
    let cases = [
        PersistedWindowBounds {
            x: -1920.0,
            y: 0.0,
            width: 800.0,
            height: 600.0,
        },
        PersistedWindowBounds {
            x: 0.0,
            y: 0.0,
            width: 9000.0,
            height: 600.0,
        },
    ];
    for bounds in &cases {
        let config = AppConfig {
            window_bounds: Some(bounds.clone()),
            ..AppConfig::default()
        };
        assert!(
            config.normalized().window_bounds.is_some(),
            "plausible bounds {bounds:?} must survive normalization"
        );
    }
}
