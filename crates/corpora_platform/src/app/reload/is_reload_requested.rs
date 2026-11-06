#[allow(unused_imports)]
use super::*;

pub fn is_reload_requested() -> bool {
    RELOAD_REQUESTED.load(Ordering::SeqCst)
}
