#[allow(unused_imports)]
use super::*;

/// Error messages may embed io paths.
pub(crate) fn scrub_home(text: &str) -> String {
    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .unwrap_or_default();
    scrub_home_with(text, &home)
}
