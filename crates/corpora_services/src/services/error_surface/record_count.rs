#[allow(unused_imports)]
use super::*;

/// Returns the number of error records without cloning the records Vec.
pub fn record_count(cx: &App) -> usize {
    cx.try_global::<ErrorSurfaceState>()
        .map(|state| state.records.len())
        .unwrap_or_default()
}
