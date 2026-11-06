#[allow(unused_imports)]
use super::*;

pub fn initialize(cx: &mut App) {
    let snapshot = TelemetrySnapshot::default();
    let runtime = TelemetryRuntime {
        sink: Arc::new(DisabledSink),
    };
    set_capability(&snapshot, cx);
    cx.set_global(snapshot);
    cx.set_global(runtime);
}
