#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn empty_locale_fires_a_locale_lint() {
    let cfg = AppConfig {
        locale: String::new(),
        ..AppConfig::default()
    };
    let lints = validate_config(&cfg);
    assert!(has_lint_for(&lints, "locale"));
}
