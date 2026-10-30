#[allow(unused_imports)]
use super::*;

#[derive(Clone)]
pub(crate) struct TelemetryRuntime {
    pub(crate) sink: Arc<dyn TelemetrySink>,
}

impl Global for TelemetryRuntime {}
