#[allow(unused_imports)]
use super::*;

/// Flush pending telemetry and permanently shut down the global tracer
/// provider. Safe to call repeatedly; later calls are no-ops.
pub fn shutdown(cx: &mut App) {
    let state = snapshot(cx);
    tracing::debug!(
        target: "corpora::telemetry",
        enabled = state.enabled,
        mode = ?state.mode,
        events_recorded = state.events_recorded,
        "telemetry shutdown requested"
    );
    flush(cx);
    // The only place shutdown_tracer_provider() may be called.
    global::shutdown_tracer_provider();
}
