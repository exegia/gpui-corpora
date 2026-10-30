#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn reconnect_policy_default_limits_retries() {
    let policy = ReconnectPolicy::default();
    assert_eq!(policy.max_retries, 5);
}
