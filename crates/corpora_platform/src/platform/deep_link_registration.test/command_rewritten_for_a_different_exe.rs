#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn command_rewritten_for_a_different_exe() {
    let stored = "\"C:\\target\\debug\\corpora.exe\" \"%1\"";
    assert!(!command_matches(
        stored,
        Path::new("C:\\target\\release\\corpora.exe")
    ));
}
