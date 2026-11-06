#[allow(unused_imports)]
use super::*;

/// Record a recent error message so it can be attached to the next crash
/// report. Keeps at most 20 entries (oldest are dropped).
pub fn track_recent_error(msg: String) {
    let slot = RECENT_ERRORS.get_or_init(|| Mutex::new(Vec::new()));
    if let Ok(mut guard) = slot.lock() {
        guard.push(msg);
        if guard.len() > 20 {
            guard.remove(0);
        }
    }
}
