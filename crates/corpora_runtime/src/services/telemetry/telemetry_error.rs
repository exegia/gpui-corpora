#[allow(unused_imports)]
use super::*;

#[derive(Debug, thiserror::Error)]
pub enum TelemetryError {
    #[error("telemetry not available: {0}")]
    NotAvailable(String),
    #[error("OTLP error: {0}")]
    Otlp(#[source] Box<dyn std::error::Error + Send + Sync>),
}
