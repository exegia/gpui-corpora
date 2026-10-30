#[allow(unused_imports)]
use super::*;

pub(crate) fn request_shutdown() {
    SHUTDOWN_REQUESTED.store(true, Ordering::SeqCst);
}
