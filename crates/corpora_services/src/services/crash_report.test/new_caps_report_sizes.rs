#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn new_caps_report_sizes() {
    let long_error = "x".repeat(10_000);
    let many_errors: Vec<String> = (0..50)
        .map(|i| format!("err{i}-{}", "y".repeat(600)))
        .collect();
    let report = CrashReport::new(long_error.clone(), long_error, false, many_errors);
    assert_eq!(report.panic_message.chars().count(), 2048);
    assert_eq!(report.backtrace.chars().count(), 8192);
    assert_eq!(report.recent_errors.len(), 20);
    assert_eq!(report.recent_errors[0].chars().count(), 512);
}
