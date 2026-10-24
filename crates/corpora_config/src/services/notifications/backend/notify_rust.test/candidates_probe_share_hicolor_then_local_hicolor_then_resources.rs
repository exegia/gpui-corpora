#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn candidates_probe_share_hicolor_then_local_hicolor_then_resources() {
    let exe_dir = Path::new("/opt/app/bin");

    let candidates = icon_candidates(exe_dir);

    assert_eq!(candidates.len(), 11);
    assert_eq!(
        candidates[0],
        Path::new("/opt/app/share/icons/hicolor/48x48/apps/corpora.png")
    );
    assert_eq!(
        candidates[4],
        Path::new("/opt/app/share/icons/hicolor/512x512/apps/corpora.png")
    );
    assert_eq!(
        candidates[5],
        Path::new("/opt/app/bin/hicolor/48x48/apps/corpora.png")
    );
    assert_eq!(
        candidates[9],
        Path::new("/opt/app/bin/hicolor/512x512/apps/corpora.png")
    );
    assert_eq!(
        candidates[10],
        Path::new("/opt/app/bin/resources/corpora.png")
    );
}
