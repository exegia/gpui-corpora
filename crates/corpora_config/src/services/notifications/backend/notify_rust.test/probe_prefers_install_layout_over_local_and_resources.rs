#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn probe_prefers_install_layout_over_local_and_resources() {
    let exe_dir = temp_root("layout-order").join("bin");
    stage_png(&exe_dir.join("hicolor/64x64/apps/corpora.png"));
    stage_png(&exe_dir.join("resources/corpora.png"));

    assert_eq!(
        probe_app_icon(&exe_dir),
        exe_dir
            .join("hicolor/64x64/apps/corpora.png")
            .display()
            .to_string()
    );
}
