#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn shortcut_is_written_and_reused() {
    let programs = temp_programs("reuse");
    let exe = fake_exe(&programs, "app.exe");

    ensure_shortcut_at(&programs, "com.example.app", &exe).expect("first write");
    let lnk = programs.join("corpora.lnk");
    assert!(lnk.exists(), "shortcut must exist after ensure");

    // Idempotent: a second run with identical inputs must not rewrite the
    // file (mtime unchanged).
    let before = std::fs::metadata(&lnk)
        .expect("metadata")
        .modified()
        .expect("mtime");
    std::thread::sleep(std::time::Duration::from_millis(20));
    ensure_shortcut_at(&programs, "com.example.app", &exe).expect("second run");
    let after = std::fs::metadata(&lnk)
        .expect("metadata")
        .modified()
        .expect("mtime");
    assert_eq!(
        before, after,
        "unchanged exe + AUMID must leave the shortcut untouched"
    );
}
