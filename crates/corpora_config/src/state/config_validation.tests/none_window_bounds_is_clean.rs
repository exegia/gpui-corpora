#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn none_window_bounds_is_clean() {
    let cfg = AppConfig {
        window_bounds: None,
        ..AppConfig::default()
    };
    let lints = validate_config(&cfg);
    assert!(lints.iter().all(|l| !l.field.starts_with("window_bounds")));
}
