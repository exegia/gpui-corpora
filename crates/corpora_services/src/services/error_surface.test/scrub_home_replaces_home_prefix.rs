#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn scrub_home_replaces_home_prefix() {
    assert_eq!(
        scrub_home_with("/home/me/logs/x.log", "/home/me"),
        "~/logs/x.log"
    );
    // Replacement is textual (matches the crash_report scrub), so a shared
    // prefix without a path boundary is also rewritten — still scrubbed.
    assert_eq!(scrub_home_with("/home/me2/x", "/home/me"), "~2/x");
    assert_eq!(scrub_home_with("plain text", ""), "plain text");
}
