#[allow(unused_imports)]
use super::*;

pub(crate) fn block_name(block: &str) -> &str {
    block
        .lines()
        .find_map(|l| l.trim().strip_prefix("name = "))
        .map(|n| n.trim_matches('"'))
        .unwrap_or_default()
}
