use std::sync::Arc;

use gpui::{App, BorrowAppContext as _, SharedString};

use super::backend_service::{
    LOG, NativeNotificationState, NotificationRuntimeSnapshot, NotificationService,
};
use super::inbox::{self, NotificationAttemptRecord, NotificationInboxKind};
use super::types::{
    NotificationBackendKind, NotificationImportance, NotificationPermissionState,
    NotificationRequest, NotificationSendResult,
};

mod apply_send_result;
mod apply_send_result_static;
mod initialize;
mod mutate_snapshot;
mod open_system_settings;
mod push_in_app_feedback;
mod refresh_daemon_state;
mod refresh_permission_state;
mod request_permission;
mod send;
mod set_native_notifications_enabled;
mod snapshot;

use apply_send_result::apply_send_result;
pub use apply_send_result_static::apply_send_result_static;
pub use initialize::initialize;
use mutate_snapshot::mutate_snapshot;
pub use open_system_settings::open_system_settings;
use push_in_app_feedback::push_in_app_feedback;
pub use refresh_daemon_state::refresh_daemon_state;
pub use refresh_permission_state::refresh_permission_state;
pub use request_permission::request_permission;
pub use send::send;
pub use set_native_notifications_enabled::set_native_notifications_enabled;
pub use snapshot::snapshot;
