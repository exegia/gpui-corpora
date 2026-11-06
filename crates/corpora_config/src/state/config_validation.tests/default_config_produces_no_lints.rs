#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn default_config_produces_no_lints() {
    let cfg = AppConfig::default();
    let lints = validate_config(&cfg);
    assert!(
        lints.is_empty(),
        "default config should be clean, got: {lints:?}"
    );
}
