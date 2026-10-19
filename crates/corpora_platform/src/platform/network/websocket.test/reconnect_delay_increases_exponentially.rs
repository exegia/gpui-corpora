#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn reconnect_delay_increases_exponentially() {
    let policy = ReconnectPolicy::default();
    let d0 = policy.delay_for_attempt(0);
    let d1 = policy.delay_for_attempt(1);
    let d2 = policy.delay_for_attempt(2);
    assert!(d1 > d0, "delay should grow with each attempt");
    assert!(d2 > d1, "delay should grow with each attempt");
}
