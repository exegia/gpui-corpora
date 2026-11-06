#[allow(unused_imports)]
use super::*;

/// Install an OTLP HTTP tracer provider on the global pipeline. Returns a
/// human-readable error when the exporter cannot be built; the returned
/// provider is kept for `force_flush` without a global shutdown.
#[cfg(feature = "otlp")]
pub(crate) fn install_otlp_tracer(
    endpoint: &str,
) -> Result<opentelemetry_sdk::trace::TracerProvider, TelemetryError> {
    use opentelemetry::KeyValue;
    use opentelemetry::global;
    use opentelemetry_otlp::WithExportConfig;
    use opentelemetry_sdk::Resource;
    use opentelemetry_sdk::propagation::TraceContextPropagator;
    use opentelemetry_sdk::runtime::Tokio;

    use super::super::SERVICE_NAME;

    if let Some(existing) = INSTALLED_PIPELINE.get() {
        tracing::warn!(
            target: "corpora::telemetry",
            requested_endpoint = %endpoint,
            "OTLP pipeline already installed; reusing it (endpoint changes need an app restart)"
        );
        return Ok(existing.clone());
    }

    let exporter = opentelemetry_otlp::new_exporter()
        .http()
        .with_endpoint(format!("{endpoint}/v1/traces"));

    let provider = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(exporter)
        .with_trace_config(opentelemetry_sdk::trace::Config::default().with_resource(
            Resource::new(vec![KeyValue::new("service.name", SERVICE_NAME)]),
        ))
        .install_batch(Tokio)
        .map_err(|e| TelemetryError::Otlp(Box::new(e)))?;

    global::set_text_map_propagator(TraceContextPropagator::new());
    // Keep the original for force_flush; the global registry holds a clone.
    global::set_tracer_provider(provider.clone());

    tracing::info!(
        target: "corpora::telemetry",
        endpoint = %endpoint,
        "OTLP tracer provider installed"
    );
    let _ = INSTALLED_PIPELINE.set(provider.clone());
    Ok(provider)
}

/// No-op fallback when the `otlp` feature is disabled — tracing-only export.
#[cfg(not(feature = "otlp"))]
pub(crate) fn install_otlp_tracer(endpoint: &str) -> Result<(), TelemetryError> {
    tracing::debug!(
        target: "corpora::telemetry",
        endpoint = %endpoint,
        "OTLP export skipped (otlp feature disabled); endpoint noted for future use"
    );
    Ok(())
}
