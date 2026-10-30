#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn resolve_endpoint_prefers_explicit() {
    let ep = resolve_otlp_endpoint(Some("https://custom.example.com:4318"));
    assert_eq!(ep, "https://custom.example.com:4318");
}
