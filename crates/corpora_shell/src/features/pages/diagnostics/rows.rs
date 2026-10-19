use gpui::App;

use crate::{
    accessibility, app_state, capabilities, commands, connectivity, crash_report, desktop_actions,
    error_surface,
    lifecycle::{LifecycleStage, LifecycleState},
    logging, notifications, secure_storage, session, shortcuts, storage, telemetry, undo_stack,
};

use super::{capability_row, row};

mod build_diagnostic_rows;
mod yes_no;

pub use build_diagnostic_rows::build_diagnostic_rows;
use yes_no::yes_no;
