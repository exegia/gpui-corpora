#[allow(unused_imports)]
use super::*;

/// Without `endpoint`, `OTEL_EXPORTER_OTLP_ENDPOINT` (or the built-in
/// default) is used. Only `Remote` + consent installs the OTLP exporter.
pub fn set_mode(mode: TelemetryMode, consented: bool, endpoint: Option<&str>, cx: &mut App) {
    let resolved = resolve_otlp_endpoint(endpoint);
    let endpoint_redacted = redact_endpoint(&resolved);
    let enabled = consented && mode != TelemetryMode::Disabled;

    let (sink, connection_error): (Arc<dyn TelemetrySink>, Option<String>) =
        match (&mode, consented) {
            (TelemetryMode::Disabled, _) | (_, false) => (Arc::new(DisabledSink), None),
            (TelemetryMode::LocalOnly, true) => (Arc::new(LocalSink), None),
            (TelemetryMode::Remote, true) => {
                // install_batch spawns its batching task on the ambient tokio
                // runtime; GPUI threads run outside any, so hand it the shared one.
                let shared = crate::services::tokio_runtime::handle(cx);
                let sink = RemoteSink::new(&resolved, shared.map(|rt| rt.handle().clone()));
                let err = if sink.connected {
                    None
                } else {
                    Some(format!("failed to connect OTLP exporter to {resolved}"))
                };
                (Arc::new(sink), err)
            }
        };

    let next = TelemetrySnapshot {
        compiled: true,
        consented,
        enabled,
        mode: mode.clone(),
        endpoint_redacted,
        events_recorded: snapshot(cx).events_recorded,
        last_export_error: connection_error,
        last_error: None,
    };

    tracing::info!(
        target: "corpora::telemetry",
        consented = next.consented,
        enabled = next.enabled,
        mode = ?next.mode,
        endpoint = ?next.endpoint_redacted,
        "telemetry mode updated"
    );

    set_capability(&next, cx);
    cx.update_global::<TelemetrySnapshot, _>(|snap, _cx| {
        *snap = next;
    });
    cx.set_global(TelemetryRuntime { sink });
}
