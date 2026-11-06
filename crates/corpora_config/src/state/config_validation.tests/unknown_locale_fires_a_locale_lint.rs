#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn unknown_locale_fires_a_locale_lint() {
    let cfg = AppConfig {
        locale: "klingon".to_string(),
        ..AppConfig::default()
    };
    let lints = validate_config(&cfg);
    assert!(has_lint_for(&lints, "locale"));
    assert!(
        lints
            .iter()
            .any(|l| l.field == "locale" && l.message.contains("klingon"))
    );
}
