#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn track_recent_error_keeps_limit() {
    for i in 0..25 {
        track_recent_error(format!("err-{i}"));
    }
    let slot = RECENT_ERRORS.get().unwrap();
    let guard = slot.lock().unwrap();
    assert!(guard.len() <= 20, "ring buffer must cap at 20 entries");
    assert_eq!(guard.last().map(String::as_str), Some("err-24"));
}
