#[allow(unused_imports)]
use super::*;

pub(crate) fn debounce_test_lock() -> std::sync::MutexGuard<'static, ()> {
    DEBOUNCE_TESTS
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}
