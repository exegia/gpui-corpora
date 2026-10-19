use ely_gpui_component::{
    buttons::{Button, SegmentedControl},
    forms::Switch,
    primitives::IconName,
    settings::{SettingsRow, SettingsSection},
};
use gpui::{App, IntoElement};

use crate::app::{self, LOCALE_EN, LOCALE_ZH_CN, theme::ThemeMode};

mod section;
mod shortcuts_section;
mod storage_section;

pub use section::section;
pub use shortcuts_section::shortcuts_section;
pub use storage_section::storage_section;
