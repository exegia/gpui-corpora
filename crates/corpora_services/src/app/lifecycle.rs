use std::fs;
use std::io::Write as _;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Mutex, OnceLock};

use gpui::{App, Global};
use serde::{Deserialize, Serialize};

use crate::time::AppTimestamp;

std::thread_local! {
    static IN_RENDER_PATH: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

#[cfg(test)]
#[path = "lifecycle.test/mod.rs"]
mod lifecycle_test;

mod app_data_dir;
mod check_previous_crash;
mod crash_marker_path;
mod enter_render_path;
mod in_render_path;
mod install_panic_hook;
mod last_panic_summary;
mod lifecycle_stage;
mod lifecycle_state;
mod recent_errors;
mod remove_crash_marker;
mod render_panic_occurred;
mod render_path_guard;
mod set_app_data_dir;
mod set_shutdown_error;
mod set_shutdown_step;
mod set_stage;
mod set_startup_error;
mod set_startup_step;
mod take_render_panic;
mod track_recent_error;
mod write_crash_marker;

use app_data_dir::APP_DATA_DIR;
pub use check_previous_crash::check_previous_crash;
use crash_marker_path::crash_marker_path;
pub use enter_render_path::enter_render_path;
pub use in_render_path::in_render_path;
pub use install_panic_hook::install_panic_hook;
use last_panic_summary::LAST_PANIC_SUMMARY;
pub use last_panic_summary::last_panic_summary;
pub use lifecycle_stage::LifecycleStage;
pub use lifecycle_state::LifecycleState;
use recent_errors::RECENT_ERRORS;
pub use remove_crash_marker::remove_crash_marker;
use render_panic_occurred::RENDER_PANIC_OCCURRED;
use render_path_guard::RenderPathGuard;
pub use set_app_data_dir::set_app_data_dir;
pub use set_shutdown_error::set_shutdown_error;
pub use set_shutdown_step::set_shutdown_step;
pub use set_stage::set_stage;
pub use set_startup_error::set_startup_error;
pub use set_startup_step::set_startup_step;
pub use take_render_panic::take_render_panic;
pub use track_recent_error::track_recent_error;
pub use write_crash_marker::write_crash_marker;
