#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn beta_update_channel_is_clean() {
    let cfg = AppConfig {
        update_channel: "beta".to_string(),
        ..AppConfig::default()
    };
    let lints = validate_config(&cfg);
    assert!(!has_lint_for(&lints, "update_channel"));
}
