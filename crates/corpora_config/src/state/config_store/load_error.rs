#[allow(unused_imports)]
use super::*;

/// Error from the last state-file load, if any; `None` before
/// [`initialize`] and after a clean load.
pub fn load_error(cx: &App) -> Option<String> {
    cx.try_global::<AppState>()
        .and_then(|s| s.last_load_error.clone())
}
