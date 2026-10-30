#[allow(unused_imports)]
use super::*;

pub fn flush(cx: &mut App) {
    with_runtime(cx, |runtime, cx| {
        cx.update_global::<TelemetrySnapshot, _>(|snap, _cx| {
            if let Err(err) = runtime.sink.flush() {
                snap.last_export_error = Some(err.to_string());
                snap.last_error = Some(err.to_string());
            }
        });
    });
}
