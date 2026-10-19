#[allow(unused_imports)]
use super::*;

/// Error from the most recent save, if any; cleared by the next
/// successful flush.
pub fn save_error(cx: &App) -> Option<String> {
    cx.try_global::<AppState>()
        .and_then(|s| s.last_save_error.clone())
}
