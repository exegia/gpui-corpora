#[allow(unused_imports)]
use super::*;

pub fn shutdown(cx: &mut App) {
    if let Some(runtime) = cx
        .try_global::<LoggingState>()
        .map(|state| state.runtime.clone())
    {
        tracing::debug!(
            target: "corpora::logging",
            enabled = runtime.enabled,
            "logging shutdown requested"
        );
        cx.update_global::<LoggingState, _>(|state, _cx| {
            state.runtime.has_guard = false;
            state.guard = None;
        });
    }
}
