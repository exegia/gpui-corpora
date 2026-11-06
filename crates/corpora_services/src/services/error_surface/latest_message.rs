#[allow(unused_imports)]
use super::*;

/// Returns the message of the latest error record, without cloning the full record.
pub fn latest_message(cx: &App) -> Option<String> {
    cx.try_global::<ErrorSurfaceState>()
        .and_then(|state| state.records.first().map(|r| r.message.clone()))
}
