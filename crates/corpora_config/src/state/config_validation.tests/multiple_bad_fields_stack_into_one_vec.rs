#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn multiple_bad_fields_stack_into_one_vec() {
    let cfg = AppConfig {
        active_path: String::new(),
        locale: "klingon".to_string(),
        update_channel: "weird".to_string(),
        ..AppConfig::default()
    };
    let lints = validate_config(&cfg);
    assert!(has_lint_for(&lints, "active_path"));
    assert!(has_lint_for(&lints, "locale"));
    assert!(has_lint_for(&lints, "update_channel"));
    assert!(lints.len() >= 3);
}
