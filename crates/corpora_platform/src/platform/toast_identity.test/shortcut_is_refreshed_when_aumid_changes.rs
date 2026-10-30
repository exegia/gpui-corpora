#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn shortcut_is_refreshed_when_aumid_changes() {
    let programs = temp_programs("aumid-changes");
    let exe = fake_exe(&programs, "app.exe");

    ensure_shortcut_at(&programs, "com.old.app", &exe).expect("write old AUMID");
    ensure_shortcut_at(&programs, "com.new.app", &exe).expect("rewrite AUMID");

    let (target, aumid) = read_back(&programs);
    assert!(
        same_path(&target, &exe),
        "shortcut must follow the current exe: {} != {}",
        target.display(),
        exe.display()
    );
    assert_eq!(aumid.as_deref(), Some("com.new.app"));
}
