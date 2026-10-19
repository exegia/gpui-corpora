#[allow(unused_imports)]
use super::*;

pub(crate) fn scrub(text: &str, max_chars: usize) -> String {
    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .unwrap_or_default();
    let scrubbed = if home.is_empty() {
        text.to_string()
    } else {
        text.replace(&home, "~")
    };
    scrubbed.chars().take(max_chars).collect()
}
