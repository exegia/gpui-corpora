#[allow(unused_imports)]
use super::*;

/// The trait is async but the SQLite bodies resolve on their first poll, so a
/// plain tokio runtime drives the futures in tests (same pattern as the
/// notification-backend tests).
pub(crate) fn block_on<F: std::future::Future>(future: F) -> F::Output {
    tokio::runtime::Runtime::new()
        .expect("tokio runtime for test")
        .block_on(future)
}
