#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn command_rewritten_when_placeholder_is_missing_or_altered() {
    let exe = Path::new("C:\\bin\\corpora.exe");
    assert!(!command_matches("\"C:\\bin\\corpora.exe\"", exe));
    assert!(!command_matches("\"C:\\bin\\corpora.exe\" \"%9\"", exe));
    assert!(!command_matches("\"C:\\bin\\corpora.exe\" %1", exe));
}
