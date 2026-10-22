#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn args_substitute_into_the_translation() {
    rust_i18n::set_locale("en");
    assert_eq!(
        localize_with_args("about_version", &[("version", "1.2.3")]),
        "Version 1.2.3"
    );
}
