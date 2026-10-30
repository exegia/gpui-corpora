use corpora_ui::ely::{
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
mod test_section;

pub use section::section;
use status_row::status_row;
use test_button::test_button;
pub use test_section::test_section;
