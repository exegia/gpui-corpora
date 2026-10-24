#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn normalized_drops_implausible_window_bounds() {
    let cases = [
        PersistedWindowBounds {
            x: 0.0,
            y: 0.0,
            width: f32::NAN,
            height: 600.0,
        },
        PersistedWindowBounds {
            x: 0.0,
            y: 0.0,
            width: f32::INFINITY,
            height: 600.0,
        },
        PersistedWindowBounds {
            x: 0.0,
            y: 0.0,
            width: -800.0,
            height: 600.0,
        },
        PersistedWindowBounds {
            x: 0.0,
            y: 0.0,
            width: MAX_PLAUSIBLE_DIM * 10.0,
            height: 600.0,
        },
    ];

    for bounds in &cases {
        let config = AppConfig {
            window_bounds: Some(bounds.clone()),
            ..AppConfig::default()
        };
        assert_eq!(
            config.normalized().window_bounds,
            None,
            "implausible bounds {bounds:?} must fall back to default placement"
        );
    }
}
