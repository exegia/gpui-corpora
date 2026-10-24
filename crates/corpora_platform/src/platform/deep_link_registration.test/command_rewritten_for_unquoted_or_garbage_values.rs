#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn command_rewritten_for_unquoted_or_garbage_values() {
    let exe = Path::new("C:\\bin\\corpora.exe");
    assert!(!command_matches("C:\\bin\\corpora.exe \"%1\"", exe));
    assert!(!command_matches("", exe));
    assert!(!command_matches("URL:Corpora", exe));
}
