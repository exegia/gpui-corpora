#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn missing_key_with_args_renders_the_key_itself() {
    assert_eq!(
        localize_with_args("i18n_no_such_key", &[("version", "1.0.0")]),
        "i18n_no_such_key"
    );
}
