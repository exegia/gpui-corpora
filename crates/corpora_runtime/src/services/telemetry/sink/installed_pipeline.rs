#[allow(unused_imports)]
use super::*;

// Holds the installed pipeline; rebuilding on every set_mode(Remote) would
// leak the previous batch exporter (background thread + connections).
#[cfg(feature = "otlp")]
pub(crate) static INSTALLED_PIPELINE: OnceLock<opentelemetry_sdk::trace::TracerProvider> =
    OnceLock::new();
