use ely_gpui_component::{
    buttons::{Button, ButtonVariant},
    forms::Switch,
    settings::{SettingsRow, SettingsSection},
};
use gpui::{IntoElement, SharedString};

use crate::notifications::{
    self, NotificationPermissionState, NotificationRequest, NotificationRuntimeSnapshot,
};

mod section;
mod status_row;
mod test_button;

pub use section::section;
use status_row::status_row;
use test_button::test_button;
