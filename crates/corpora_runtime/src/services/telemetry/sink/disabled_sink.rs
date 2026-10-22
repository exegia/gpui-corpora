#[allow(unused_imports)]
use super::*;

#[derive(Default)]
pub(crate) struct DisabledSink;

impl super::super::TelemetrySink for DisabledSink {
    fn record_event(&self, _name: &str) -> Result<(), TelemetryError> {
        Ok(())
    }

    fn record_error(&self, _error: &str) -> Result<(), TelemetryError> {
        Ok(())
    }

    fn set_user_properties(&self, _key: &str, _value: &str) -> Result<(), TelemetryError> {
        Ok(())
    }

    fn flush(&self) -> Result<(), TelemetryError> {
        Ok(())
    }
}
