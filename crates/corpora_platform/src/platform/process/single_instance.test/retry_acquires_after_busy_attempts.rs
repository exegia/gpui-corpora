#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn retry_acquires_after_busy_attempts() {
    let mut calls = 0;
    let outcome = retry_until_single(
        || {
            calls += 1;
            if calls <= 3 {
                AcquireOutcome::Busy
            } else {
                AcquireOutcome::Acquired(())
            }
        },
        10,
        Duration::from_millis(1),
    );
    assert!(matches!(outcome, AcquireOutcome::Acquired(())));
    assert_eq!(calls, 4);
}
