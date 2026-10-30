#[allow(unused_imports)]
use super::*;

/// Wait after the last config mutation before flushing, so bursts of
/// `update_config` calls coalesce into a single write.
pub(crate) const DEBOUNCE_MS: u64 = 300;
