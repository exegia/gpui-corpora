#[allow(unused_imports)]
use super::*;

pub(crate) fn scrub_home_with(text: &str, home: &str) -> String {
    if home.is_empty() {
        text.to_string()
    } else {
        text.replace(home, "~")
    }
}
