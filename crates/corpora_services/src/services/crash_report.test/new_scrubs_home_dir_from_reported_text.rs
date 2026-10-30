#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn new_scrubs_home_dir_from_reported_text() {
    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .unwrap_or_default();
    let report = CrashReport::new(
        format!("panicked at {home}/src/main.rs"),
        format!("frame 0: {home}/src/lib.rs"),
        false,
        vec![format!("error reading {home}/secret.txt")],
    );
    if home.is_empty() {
        return; // nothing to scrub without a home dir
    }
    assert!(!report.panic_message.contains(&home));
    assert!(!report.backtrace.contains(&home));
    assert!(!report.recent_errors[0].contains(&home));
    assert!(report.panic_message.contains("~/src/main.rs"));
}
