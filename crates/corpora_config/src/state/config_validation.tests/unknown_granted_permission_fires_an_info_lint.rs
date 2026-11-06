#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn unknown_granted_permission_fires_an_info_lint() {
    let mut perms = HashSet::new();
    perms.insert("notifications".to_string());
    perms.insert("mind-reading".to_string());
    let cfg = AppConfig {
        granted_permissions: perms,
        ..AppConfig::default()
    };
    let lints = validate_config(&cfg);
    assert!(
        lints
            .iter()
            .any(|l| l.field == "granted_permissions.mind-reading"
                && l.severity == LintSeverity::Info)
    );
    assert!(!has_lint_for(&lints, "granted_permissions.notifications"));
}
