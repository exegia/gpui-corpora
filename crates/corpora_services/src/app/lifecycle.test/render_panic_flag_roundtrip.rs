#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn render_panic_flag_roundtrip() {
    // The flag is process-global; consume any stale value first so the
    // assertions below are deterministic even with parallel tests.
    let _ = take_render_panic();
    RENDER_PANIC_OCCURRED.store(true, Ordering::SeqCst);
    assert!(take_render_panic(), "first read should be true");
    assert!(
        !take_render_panic(),
        "second read should be false (flag reset)"
    );
}
