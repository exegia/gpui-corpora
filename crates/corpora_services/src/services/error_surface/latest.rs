#[allow(unused_imports)]
use super::*;

pub fn latest(cx: &App) -> Option<ErrorRecord> {
    cx.try_global::<ErrorSurfaceState>()
        .and_then(|state| state.records.first().cloned())
}
