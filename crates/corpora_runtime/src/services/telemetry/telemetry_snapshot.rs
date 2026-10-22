#[allow(unused_imports)]
use super::*;

#[derive(Clone, Debug)]
pub struct TelemetrySnapshot {
    pub compiled: bool,
    pub consented: bool,
    pub enabled: bool,
    pub mode: TelemetryMode,
    pub endpoint_redacted: Option<String>,
    pub events_recorded: u64,
    pub last_export_error: Option<String>,
    pub last_error: Option<String>,
}

impl Default for TelemetrySnapshot {
    fn default() -> Self {
        Self {
            compiled: true,
            consented: false,
            enabled: false,
            mode: TelemetryMode::Disabled,
            endpoint_redacted: None,
            events_recorded: 0,
            last_export_error: None,
            last_error: None,
        }
    }
}

impl Global for TelemetrySnapshot {}
