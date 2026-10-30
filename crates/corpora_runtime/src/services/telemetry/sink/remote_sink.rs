#[allow(unused_imports)]
use super::*;

/// Remote sink exporting via OTLP over HTTP. Endpoint precedence matches
/// [`super::resolve_otlp_endpoint`]; without the `otlp` feature it stays in a
/// tracing-only mode that reports itself connected.
#[derive(Clone)]
pub(crate) struct RemoteSink {
    pub(crate) endpoint: String,
    pub(crate) connected: bool,
    /// Kept so flush can call force_flush without shutting down the global provider.
    #[cfg(feature = "otlp")]
    pub(crate) provider: Option<opentelemetry_sdk::trace::TracerProvider>,
}

impl RemoteSink {
    /// `runtime` must be the shared tokio runtime handle: the batch exporter
    /// spawns its batching task on the ambient runtime, and GPUI threads run
    /// outside any tokio context. Without a handle the sink stays disconnected
    /// instead of panicking.
    pub(crate) fn new(endpoint: &str, runtime: Option<tokio::runtime::Handle>) -> Self {
        #[cfg(feature = "otlp")]
        let (connected, provider) = Self::install(endpoint, runtime);
        // The () install payload only exists without the `otlp` feature.
        #[cfg(not(feature = "otlp"))]
        let (connected, ()) = Self::install(endpoint, runtime);
        Self {
            endpoint: endpoint.to_owned(),
            connected,
            #[cfg(feature = "otlp")]
            provider,
        }
    }

    #[cfg(feature = "otlp")]
    pub(crate) fn install(
        endpoint: &str,
        runtime: Option<tokio::runtime::Handle>,
    ) -> (bool, Option<opentelemetry_sdk::trace::TracerProvider>) {
        let Some(handle) = runtime else {
            tracing::warn!(
                target: "corpora::telemetry",
                "no tokio runtime handle available; OTLP exporter not installed"
            );
            return (false, None);
        };
        let _enter = handle.enter();
        match install_otlp_tracer(endpoint) {
            Ok(provider) => (true, Some(provider)),
            Err(err) => {
                tracing::warn!(
                    target: "corpora::telemetry",
                    endpoint = %endpoint,
                    error = %err,
                    "OTLP tracer provider installation failed; events will be logged locally"
                );
                (false, None)
            }
        }
    }

    #[cfg(not(feature = "otlp"))]
    pub(crate) fn install(endpoint: &str, _runtime: Option<tokio::runtime::Handle>) -> (bool, ()) {
        let connected = match install_otlp_tracer(endpoint) {
            Ok(()) => true,
            Err(err) => {
                tracing::warn!(
                    target: "corpora::telemetry",
                    endpoint = %endpoint,
                    error = %err,
                    "OTLP tracer provider installation failed; events will be logged locally"
                );
                false
            }
        };
        (connected, ())
    }

    /// Push buffered spans to the collector without disabling the provider.
    #[cfg(feature = "otlp")]
    pub(crate) fn force_flush_provider(&self) -> Result<(), TelemetryError> {
        let Some(provider) = self.provider.as_ref() else {
            tracing::warn!(
                target: "corpora::telemetry",
                "force_flush called but no SDK provider is available; falling back to no-op"
            );
            return Ok(());
        };

        let errors: Vec<_> = provider
            .force_flush()
            .into_iter()
            .filter_map(|r| r.err())
            .collect();

        if errors.is_empty() {
            Ok(())
        } else {
            let msg = errors
                .iter()
                .map(|e| e.to_string())
                .collect::<Vec<_>>()
                .join(", ");
            Err(TelemetryError::Otlp(Box::new(std::io::Error::other(
                format!("force_flush errors: {msg}"),
            ))))
        }
    }

    #[cfg(not(feature = "otlp"))]
    pub(crate) fn force_flush_provider(&self) -> Result<(), TelemetryError> {
        Ok(())
    }

    /// Gate for methods that need collector connectivity; `log_dropped` keeps
    /// each call site's structured fields (event name, error text, key/value)
    /// in its own tracing macro.
    pub(crate) fn require_connected(
        &self,
        log_dropped: impl FnOnce(),
    ) -> Result<(), TelemetryError> {
        if self.connected {
            return Ok(());
        }
        log_dropped();
        Err(TelemetryError::NotAvailable(format!(
            "OTLP exporter not connected to {}",
            self.endpoint
        )))
    }
}

impl super::super::TelemetrySink for RemoteSink {
    fn record_event(&self, name: &str) -> Result<(), TelemetryError> {
        self.require_connected(|| {
            tracing::warn!(
                target: "corpora::telemetry",
                endpoint = %self.endpoint,
                event = %name,
                "remote telemetry event dropped (not connected)"
            );
        })?;
        tracing::debug!(target: "corpora::telemetry", endpoint = %self.endpoint, event = %name, "remote telemetry event queued");
        Ok(())
    }

    fn record_error(&self, error: &str) -> Result<(), TelemetryError> {
        self.require_connected(|| {
            tracing::warn!(
                target: "corpora::telemetry",
                endpoint = %self.endpoint,
                error = %error,
                "remote telemetry error dropped (not connected)"
            );
        })?;
        tracing::warn!(target: "corpora::telemetry", endpoint = %self.endpoint, error = %error, "remote telemetry error queued");
        Ok(())
    }

    fn set_user_properties(&self, key: &str, value: &str) -> Result<(), TelemetryError> {
        self.require_connected(|| {
            tracing::debug!(
                target: "corpora::telemetry",
                endpoint = %self.endpoint,
                key = %key,
                value = %value,
                "remote telemetry user property dropped (not connected)"
            );
        })?;
        tracing::debug!(target: "corpora::telemetry", endpoint = %self.endpoint, key = %key, value = %value, "remote telemetry user property queued");
        Ok(())
    }

    /// Flush is repeatable (Settings button) and leaves the provider active,
    /// unlike the one-way `shutdown_tracer_provider()`.
    fn flush(&self) -> Result<(), TelemetryError> {
        self.require_connected(|| {
            tracing::debug!(
                target: "corpora::telemetry",
                endpoint = %self.endpoint,
                "remote telemetry flush skipped (not connected)"
            );
        })?;
        tracing::debug!(target: "corpora::telemetry", endpoint = %self.endpoint, "remote telemetry flush");
        self.force_flush_provider()
    }
}
