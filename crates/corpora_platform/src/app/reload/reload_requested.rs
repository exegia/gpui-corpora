#[allow(unused_imports)]
use super::*;

/// Set by [`request_reload`], polled by the host after GPUI has shut down.
pub(crate) static RELOAD_REQUESTED: AtomicBool = AtomicBool::new(false);
