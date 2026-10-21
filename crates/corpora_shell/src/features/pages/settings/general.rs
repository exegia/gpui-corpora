use corpora_ui::ely::{
    buttons::{Button, SegmentedControl},
    forms::Switch,
    settings::{SettingsRow, SettingsSection},
};
use gpui::{App, IntoElement};

use crate::app::{self, LOCALE_EN, LOCALE_ZH_CN};

mod section;
mod shortcuts_section;
mod storage_section;
mod theme_picker;

pub use section::section;
pub use shortcuts_section::shortcuts_section;
pub use storage_section::storage_section;
