#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn probe_prefers_smaller_size_within_a_layout() {
    let exe_dir = temp_root("size-order").join("bin");
    stage_png(&exe_dir.join("hicolor/256x256/apps/corpora.png"));
    stage_png(&exe_dir.join("hicolor/64x64/apps/corpora.png"));

    assert_eq!(
        probe_app_icon(&exe_dir),
        exe_dir
            .join("hicolor/64x64/apps/corpora.png")
            .display()
            .to_string()
    );
}
