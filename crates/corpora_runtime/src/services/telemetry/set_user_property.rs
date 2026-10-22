#[allow(unused_imports)]
use super::*;

pub fn set_user_property(key: &str, value: &str, cx: &mut App) {
    with_runtime(cx, |runtime, cx| {
        let result = runtime.sink.set_user_properties(key, value);
        handle_record_result(result, cx);
    });
}
