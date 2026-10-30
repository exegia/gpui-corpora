#[allow(unused_imports)]
use super::*;

pub(crate) fn set_capability(snapshot: &TelemetrySnapshot, cx: &mut App) {
    crate::capabilities::set(
        "telemetry",
        crate::capabilities::CapabilityStatus {
            supported: snapshot.compiled,
            enabled: snapshot.enabled,
            degraded: snapshot.last_error.is_some(),
            reason: if !snapshot.consented {
                Some("telemetry disabled until consent".into())
            } else {
                None
            },
            last_error: snapshot.last_error.clone().map(Into::into),
        },
        cx,
    );
}
