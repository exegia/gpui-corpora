#[allow(unused_imports)]
use super::*;

#[derive(Default)]
pub(crate) struct LocalSink;

impl super::super::TelemetrySink for LocalSink {
    fn record_event(&self, name: &str) -> Result<(), TelemetryError> {
        tracing::debug!(target: "corpora::telemetry", event = %name, "local telemetry event");
        Ok(())
    }

    fn record_error(&self, error: &str) -> Result<(), TelemetryError> {
        tracing::warn!(target: "corpora::telemetry", error = %error, "local telemetry error");
        Ok(())
    }

    fn set_user_properties(&self, key: &str, value: &str) -> Result<(), TelemetryError> {
        tracing::debug!(target: "corpora::telemetry", key = %key, value = %value, "local telemetry user property");
        Ok(())
    }

    fn flush(&self) -> Result<(), TelemetryError> {
        Ok(())
    }
}
