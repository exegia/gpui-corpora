#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn redacts_endpoint_host_only() {
    let value =
        redact_endpoint("https://telemetry.example.com/v1/events").expect("redacted endpoint");
    assert_eq!(value, "telemetry.example.com/…");
}
