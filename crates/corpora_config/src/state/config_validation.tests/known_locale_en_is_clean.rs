#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn known_locale_en_is_clean() {
    let cfg = AppConfig {
        locale: "en".to_string(),
        ..AppConfig::default()
    };
    let lints = validate_config(&cfg);
    assert!(!has_lint_for(&lints, "locale"));
}
