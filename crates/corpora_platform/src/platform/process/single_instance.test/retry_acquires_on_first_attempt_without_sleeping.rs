#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn retry_acquires_on_first_attempt_without_sleeping() {
    let mut calls = 0;
    let start = std::time::Instant::now();
    let outcome = retry_until_single(
        || {
            calls += 1;
            AcquireOutcome::Acquired("lock")
        },
        50,
        Duration::from_secs(30),
    );
    assert!(matches!(outcome, AcquireOutcome::Acquired("lock")));
    assert_eq!(calls, 1);
    // A first-try success must not pay the sleep interval.
    assert!(start.elapsed() < Duration::from_secs(30));
}
