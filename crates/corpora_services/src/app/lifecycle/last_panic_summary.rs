#[allow(unused_imports)]
use super::*;

pub(crate) static LAST_PANIC_SUMMARY: OnceLock<Mutex<Option<String>>> = OnceLock::new();

pub fn last_panic_summary() -> Option<String> {
    LAST_PANIC_SUMMARY
        .get()
        .and_then(|slot| slot.lock().ok().and_then(|value| value.clone()))
}
