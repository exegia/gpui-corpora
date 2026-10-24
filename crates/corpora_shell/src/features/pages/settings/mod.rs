//! Settings: sections listed beside the open one. The open section is the
//! router's `:section` param, so `corpora://settings/notifications` opens it.

mod developer;
mod general;
mod notifications_section;
mod workspace;

use ely_gpui_component::{
    buttons::Button, primitives::IconName, settings::SettingsLayout, typography::Heading,
};
use gpui::{prelude::*, *};
use gpui_router::{use_navigate, use_params};

use crate::app::LocaleState;
use crate::ids::EventId;
use crate::notifications::{self, NativeNotificationState, NotificationRuntimeSnapshot};
use crate::shell::Workspace;

mod button_row;
mod event_log_cap;
mod sections;
mod settings_page;

use button_row::button_row;
use event_log_cap::EVENT_LOG_CAP;
use sections::SECTIONS;
pub use settings_page::SettingsPage;
