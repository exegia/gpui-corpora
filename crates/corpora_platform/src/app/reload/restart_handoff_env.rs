#[allow(unused_imports)]
use super::*;

/// Set child-only by the Windows spawner so preflight retries the instance
/// lock while the old process dies.
pub const RESTART_HANDOFF_ENV: &str = "CORPORA_RESTART_HANDOFF";
