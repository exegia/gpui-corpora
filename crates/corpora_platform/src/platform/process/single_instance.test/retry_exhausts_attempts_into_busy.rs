#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn retry_exhausts_attempts_into_busy() {
    let mut calls = 0;
    let outcome: AcquireOutcome<()> = retry_until_single(
        || {
            calls += 1;
            AcquireOutcome::Busy
        },
        5,
        Duration::from_millis(1),
    );
    assert!(matches!(outcome, AcquireOutcome::Busy));
    assert_eq!(calls, 5);
}
