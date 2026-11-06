#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn probe_falls_back_to_theme_name_when_nothing_is_bundled() {
    let exe_dir = temp_root("fallback").join("bin");

    assert_eq!(probe_app_icon(&exe_dir), APP_ICON_NAME);
}
