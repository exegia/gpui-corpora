#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn resolve_endpoint_falls_back_to_default() {
    // Single-threaded test process: no concurrent env reads during removal.
    unsafe { std::env::remove_var(ENV_OTLP_ENDPOINT) };
    let ep = resolve_otlp_endpoint(None);
    assert_eq!(ep, DEFAULT_OTLP_ENDPOINT);
}
