#[allow(unused_imports)]
use super::*;

pub(crate) fn parameter(cx: &App, key: &str) -> String {
    use_params(cx)
        .get(key)
        .map(ToString::to_string)
        .unwrap_or_default()
}
