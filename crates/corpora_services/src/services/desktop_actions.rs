use std::{
    collections::BTreeMap,
    path::PathBuf,
    sync::{Arc, Mutex},
};

use arboard::Clipboard;
use gpui::{App, BorrowAppContext as _, Global};
use notify::{RecommendedWatcher, RecursiveMode, Watcher};

mod build_diagnostics_text;
mod copy_diagnostics;
mod copy_text;
mod desktop_action_error;
mod desktop_actions_inner;
mod desktop_actions_snapshot;
mod desktop_actions_state;
mod initialize;
mod open_config_folder;
mod open_logs_folder;
mod open_path;
mod open_url;
mod pick_file;
mod pick_folder;
mod pick_with_dialog;
mod save_file;
mod shutdown;
mod snapshot;
mod unwatch_all;
mod unwatch_path;
mod update_result;
mod watch_config_dir;
mod watch_log_dir;
mod watch_path;

use build_diagnostics_text::build_diagnostics_text;
pub use copy_diagnostics::copy_diagnostics;
pub use copy_text::copy_text;
pub use desktop_action_error::DesktopActionError;
use desktop_actions_inner::DesktopActionsInner;
pub use desktop_actions_snapshot::DesktopActionsSnapshot;
pub use desktop_actions_state::DesktopActionsState;
pub use initialize::initialize;
pub use open_config_folder::open_config_folder;
pub use open_logs_folder::open_logs_folder;
pub use open_path::open_path;
pub use open_url::open_url;
pub use pick_file::pick_file;
pub use pick_folder::pick_folder;
use pick_with_dialog::pick_with_dialog;
pub use save_file::save_file;
pub use shutdown::shutdown;
pub use snapshot::snapshot;
pub use unwatch_all::unwatch_all;
use unwatch_path::unwatch_path;
use update_result::update_result;
pub use watch_config_dir::watch_config_dir;
pub use watch_log_dir::watch_log_dir;
use watch_path::watch_path;
