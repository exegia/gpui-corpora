#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn command_matches_current_exe_ignoring_case() {
    let stored = "\"C:\\Bin\\corpora.EXE\" \"%1\"";
    assert!(command_matches(stored, Path::new("c:\\bin\\CORPORA.exe")));
}
