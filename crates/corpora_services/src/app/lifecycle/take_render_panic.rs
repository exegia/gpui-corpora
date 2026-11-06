#[allow(unused_imports)]
use super::*;

pub fn take_render_panic() -> bool {
    RENDER_PANIC_OCCURRED.swap(false, Ordering::SeqCst)
}
