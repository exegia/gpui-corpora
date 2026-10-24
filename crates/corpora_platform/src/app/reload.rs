//! Self-reload support: a flag set via [`request_reload`], polled
//! post-shutdown to relaunch the same binary with the original argv — exec()
//! in place on Unix, spawn-and-exit on Windows.

use std::sync::atomic::{AtomicBool, Ordering};

use anyhow::{Context, Result};
use tracing::{error, info};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flag_roundtrip() {
        // Reset to a known state; tests run concurrently on the same static,
        // but this process does not actually exec so a benign toggle is fine.
        RELOAD_REQUESTED.store(false, Ordering::SeqCst);
        assert!(!is_reload_requested());
        request_reload();
        assert!(is_reload_requested());
        RELOAD_REQUESTED.store(false, Ordering::SeqCst);
    }
}

mod exec_reload;
mod is_reload_requested;
mod log;
mod perform_reload;
mod reload_requested;
mod request_reload;
mod restart_handoff_env;
mod spawn_reload;

#[cfg(unix)]
use exec_reload::exec_reload;
pub use is_reload_requested::is_reload_requested;
use log::LOG;
pub use perform_reload::perform_reload;
use reload_requested::RELOAD_REQUESTED;
pub use request_reload::request_reload;
pub use restart_handoff_env::RESTART_HANDOFF_ENV;
#[cfg(windows)]
use spawn_reload::spawn_reload;
