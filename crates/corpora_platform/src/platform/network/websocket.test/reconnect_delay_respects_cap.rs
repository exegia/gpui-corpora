#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn reconnect_delay_respects_cap() {
    let policy = ReconnectPolicy {
        base_delay_ms: 1000,
        max_delay_ms: Some(5000),
        ..Default::default()
    };
    // attempt 10 would be 1000 * 2^10 = 1_024_000 ms without cap
    let capped = policy.delay_for_attempt(10);
    assert_eq!(capped.as_millis(), 5000);
}
