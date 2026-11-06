#[allow(unused_imports)]
use super::*;

/// A stored `shell\open\command` value matches when its quoted path is the
/// current exe (case-insensitive, like registry paths) and the `%1`
/// placeholder the OS substitutes the URL into is intact.
pub(crate) fn command_matches(command: &str, exe: &Path) -> bool {
    let Some((quoted_path, tail)) = command
        .strip_prefix('"')
        .and_then(|rest| rest.split_once('"'))
    else {
        return false;
    };
    tail == " \"%1\"" && same_path(Path::new(quoted_path), exe)
}
