#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn probe_finds_share_layout_beside_bin() {
    let root = temp_root("share-layout");
    let png = root.join("share/icons/hicolor/48x48/apps/corpora.png");
    stage_png(&png);

    assert_eq!(probe_app_icon(&root.join("bin")), png.display().to_string());
}
