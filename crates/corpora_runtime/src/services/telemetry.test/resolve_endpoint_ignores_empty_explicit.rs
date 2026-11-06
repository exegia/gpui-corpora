#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn resolve_endpoint_ignores_empty_explicit() {
    let ep = resolve_otlp_endpoint(Some(""));
    assert!(!ep.is_empty());
}
