use std::fmt;

use gpui::SharedString;

// App identity: three DISTINCT ids — display name, desktop-entry/bundle id,
// X11 WM_CLASS (must equal .desktop StartupWMClass).

mod action_open;
mod action_reply;
mod action_snooze;
mod app_display_name;
mod app_icon_name;
mod category_actions;
mod category_reply;
mod desktop_entry_id;
mod notification_backend_kind;
mod notification_capabilities;
mod notification_importance;
mod notification_permission_state;
mod notification_request;
mod notification_send_result;
mod wm_class;

pub use action_open::ACTION_OPEN;
pub use action_reply::ACTION_REPLY;
pub use action_snooze::ACTION_SNOOZE;
pub use app_display_name::APP_DISPLAY_NAME;
pub use app_icon_name::APP_ICON_NAME;
pub use category_actions::CATEGORY_ACTIONS;
pub use category_reply::CATEGORY_REPLY;
pub use desktop_entry_id::DESKTOP_ENTRY_ID;
pub use notification_backend_kind::NotificationBackendKind;
pub use notification_capabilities::NotificationCapabilities;
pub use notification_importance::NotificationImportance;
pub use notification_permission_state::NotificationPermissionState;
pub use notification_request::NotificationRequest;
pub use notification_send_result::NotificationSendResult;
pub use wm_class::WM_CLASS;
