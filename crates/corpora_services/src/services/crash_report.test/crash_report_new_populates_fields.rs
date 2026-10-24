#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn crash_report_new_populates_fields() {
    let report = CrashReport::new(
        "panic message".into(),
        "backtrace".into(),
        true,
        vec!["err1".into()],
    );
    assert_eq!(report.panic_message, "panic message");
    assert_eq!(report.backtrace, "backtrace");
    assert!(report.render_path);
    assert_eq!(report.recent_errors, vec!["err1"]);
    assert!(!report.id.is_empty());
    assert!(!report.app_version.is_empty());
    assert!(!report.os.is_empty());
    assert!(!report.arch.is_empty());
    assert!(!report.timestamp.is_empty());
}
