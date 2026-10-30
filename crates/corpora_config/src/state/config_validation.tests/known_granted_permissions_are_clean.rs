#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn known_granted_permissions_are_clean() {
    let mut perms = HashSet::new();
    perms.insert("notifications".to_string());
    let cfg = AppConfig {
        granted_permissions: perms,
        ..AppConfig::default()
    };
    let lints = validate_config(&cfg);
    assert!(
        lints
            .iter()
            .all(|l| !l.field.starts_with("granted_permissions"))
    );
}
