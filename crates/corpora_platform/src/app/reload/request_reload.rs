#[allow(unused_imports)]
use super::*;

/// Request an in-place relaunch after shutdown; call [`perform_reload`] only
/// once GPUI has fully torn down.
pub fn request_reload() {
    RELOAD_REQUESTED.store(true, Ordering::SeqCst);
    info!(target: LOG, "reload requested");
}
