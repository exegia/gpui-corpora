#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn unknown_update_channel_fires_a_channel_lint() {
    let cfg = AppConfig {
        update_channel: "canary-experimental".to_string(),
        ..AppConfig::default()
    };
    let lints = validate_config(&cfg);
    assert!(has_lint_for(&lints, "update_channel"));
    assert!(
        lints
            .iter()
            .any(|l| l.field == "update_channel" && l.message.contains("canary-experimental"))
    );
}
