#[allow(unused_imports)]
use super::*;

/// Recompute bridge state from the open windows and store it. Call at App
/// level (after window open/close, or deferred) — from inside a window
/// update, that window itself reports inactive because gpui takes it out of
/// the map for the duration.
pub fn refresh(cx: &mut App) {
    let total_windows = cx.windows().len();
    let mut active_windows = 0;
    for handle in cx.windows() {
        let active = handle
            .update(cx, |_, window, _| window.is_a11y_active())
            .unwrap_or(false);
        active_windows += usize::from(active);
    }
    cx.set_global(observe(active_windows, total_windows));
}
