use std::collections::HashSet;
use std::io::{Read as _, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};

use atomic_write_file::AtomicWriteFile;
use gpui::{App, BorrowAppContext, Global, Task};
// Renamed upstream at gpui-component 5a5e2ab; variant names (and so the
// persisted serde representation) are unchanged.
use serde::{Deserialize, Serialize};

use crate::paths::ensure_parent_dir;
use crate::{
    app::{LOCALE_EN, LOCALE_ZH_CN, theme::ThemeMode},
    errors::AppError,
    notifications::inbox::NotificationInboxItem,
    paths::AppPaths,
};

#[cfg(test)]
#[path = "config_store.test/mod.rs"]
mod config_store_test;

mod app_config;
mod app_state;
mod app_state_version;
mod arm_debounce;
mod commit_flush;
mod config;
mod config_handle;
mod debounce_ms;
mod default_show_frame_time;
mod default_stable;
mod default_true;
mod force_save;
mod initialize;
mod load_config;
mod load_error;
mod max_plausible_dim;
mod max_state_file_bytes;
mod oversized_state_error;
mod paths;
mod persisted_window_bounds;
mod prepare_flush;
mod quarantine_bad_config;
mod save_config;
mod save_error;
mod save_scheduled;
mod state_read_error;
mod update_channel;
mod update_config;
mod with_config;

pub use app_config::AppConfig;
pub use app_state::AppState;
pub use app_state_version::APP_STATE_VERSION;
use arm_debounce::arm_debounce;
use commit_flush::commit_flush;
pub use config::config;
pub use config_handle::config_handle;
use debounce_ms::DEBOUNCE_MS;
use default_show_frame_time::default_show_frame_time;
use default_stable::default_stable;
use default_true::default_true;
pub use force_save::force_save;
pub use initialize::initialize;
use load_config::load_config;
pub use load_error::load_error;
use max_plausible_dim::MAX_PLAUSIBLE_DIM;
use max_state_file_bytes::MAX_STATE_FILE_BYTES;
use oversized_state_error::oversized_state_error;
pub use paths::paths;
pub use persisted_window_bounds::PersistedWindowBounds;
use prepare_flush::prepare_flush;
use quarantine_bad_config::quarantine_bad_config;
use save_config::save_config;
pub use save_error::save_error;
use save_scheduled::SAVE_SCHEDULED;
use state_read_error::state_read_error;
pub use update_channel::update_channel;
pub use update_config::update_config;
pub use with_config::with_config;
