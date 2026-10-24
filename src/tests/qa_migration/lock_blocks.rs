#[allow(unused_imports)]
use super::*;

/// Returns every `[[package]]` block whose package name matches.
pub(crate) fn lock_blocks<'a>(lock: &'a str, package: &str) -> Vec<&'a str> {
    lock.split("[[package]]")
        .skip(1)
        .filter(|block| {
            block
                .lines()
                .any(|l| l.trim() == format!("name = \"{package}\""))
        })
        .collect()
}
