#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn shortcut_is_refreshed_when_exe_moves() {
    let programs = temp_programs("exe-moves");
    let exe_a = fake_exe(&programs, "app-a.exe");
    let exe_b = fake_exe(&programs, "app-b.exe");

    ensure_shortcut_at(&programs, "com.example.app", &exe_a).expect("write for exe a");
    ensure_shortcut_at(&programs, "com.example.app", &exe_b).expect("rewrite for exe b");

    let (target, aumid) = read_back(&programs);
    // read_back yields the shell-normalized long form, which never textually
    // equals the temp_dir-built path when TEMP is the 8.3 short form.
    assert!(
        same_path(&target, &exe_b),
        "shortcut must follow the current exe: {} != {}",
        target.display(),
        exe_b.display()
    );
    assert_eq!(aumid.as_deref(), Some("com.example.app"));
}
