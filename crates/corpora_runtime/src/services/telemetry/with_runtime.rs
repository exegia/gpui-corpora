#[allow(unused_imports)]
use super::*;

pub(crate) fn with_runtime(cx: &mut App, f: impl FnOnce(TelemetryRuntime, &mut App)) {
    if let Some(runtime) = cx.try_global::<TelemetryRuntime>().cloned() {
        f(runtime, cx);
    }
}
