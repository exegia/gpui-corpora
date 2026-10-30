//! The Settings window shares application preferences and owns its selected tab.

mod developer;
mod general;
mod notifications_section;
mod workspace;

use corpora_ui::ely::buttons::Button;
use gpui::{prelude::*, *};

use crate::app::LocaleState;
use crate::ids::EventId;
use crate::notifications::{self, NativeNotificationState, NotificationRuntimeSnapshot};
use crate::shell::Workspace;

mod button_row;
mod event_log_cap;
mod page;
mod pending_navigation;
mod sections;
mod settings_page;

use button_row::button_row;
use event_log_cap::EVENT_LOG_CAP;
use sections::SECTIONS;
pub use settings_page::SettingsPage;
