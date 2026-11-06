#[allow(unused_imports)]
use super::*;

/// Dispatch Quit at the recorded root window, falling back to app-level
/// dispatch. `App::dispatch_action` routes at `active_window()` — thread-local
/// `GetActiveWindow()` on Windows — which can still report a closing overlay
/// and drop the action.
pub fn dispatch_quit(cx: &mut App) {
    let routed = root_window(cx).is_some_and(|root| {
        root.update(cx, |_, window, cx| {
            window.dispatch_action(Box::new(crate::app::Quit), cx)
        })
        .is_ok()
    });
    if !routed {
        cx.dispatch_action(&crate::app::Quit);
    }
}
