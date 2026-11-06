#[allow(unused_imports)]
use super::*;

pub fn record_error(error: &str, cx: &mut App) {
    with_runtime(cx, |runtime, cx| {
        let result = runtime.sink.record_error(error);
        handle_record_result(result, cx);
    });
}
