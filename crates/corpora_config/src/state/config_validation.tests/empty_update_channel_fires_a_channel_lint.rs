#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn empty_update_channel_fires_a_channel_lint() {
    let cfg = AppConfig {
        update_channel: String::new(),
        ..AppConfig::default()
    };
    let lints = validate_config(&cfg);
    assert!(has_lint_for(&lints, "update_channel"));
}
