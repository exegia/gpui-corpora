#[allow(unused_imports)]
use super::*;

pub(crate) fn handle_record_result(result: Result<(), TelemetryError>, cx: &mut App) {
    cx.update_global::<TelemetrySnapshot, _>(|snap, _cx| match result {
        Ok(()) => {
            snap.events_recorded = snap.events_recorded.saturating_add(1);
            snap.last_error = None;
        }
        Err(err) => {
            snap.last_export_error = Some(err.to_string());
            snap.last_error = Some(err.to_string());
        }
    });
}
