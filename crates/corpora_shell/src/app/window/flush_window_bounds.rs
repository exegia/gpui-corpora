#[allow(unused_imports)]
use super::*;

/// Persist current window bounds immediately, bypassing the debounce; no-op
/// without an open window. Called from the `Quit` handler.
pub fn flush_window_bounds(cx: &mut App) {
    let Some(window_handle) = root_window(cx).or_else(|| cx.active_window()) else {
        return;
    };
    window_handle
        .update(cx, |_, window, cx| {
            let persisted = persisted_bounds(window.window_bounds().get_bounds());
            crate::app_state::update_config(cx, |config| {
                config.window_bounds = Some(persisted);
            });
        })
        .ok();
}
