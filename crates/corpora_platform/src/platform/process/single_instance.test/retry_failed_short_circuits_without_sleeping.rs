#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn retry_failed_short_circuits_without_sleeping() {
    let mut calls = 0;
    let start = std::time::Instant::now();
    let outcome: AcquireOutcome<()> = retry_until_single(
        || {
            calls += 1;
            AcquireOutcome::Failed
        },
        50,
        Duration::from_secs(30),
    );
    assert!(matches!(outcome, AcquireOutcome::Failed));
    assert_eq!(calls, 1, "Failed must not consume further attempts");
    assert!(start.elapsed() < Duration::from_secs(30));
}
