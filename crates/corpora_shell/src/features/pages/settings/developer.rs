use corpora_ui::ely::{
    buttons::{Button, ButtonVariant},
    feedback::Toast,
    forms::Switch,
    settings::{SettingsRow, SettingsSection},
    typography::Caption,
};
use gpui::{App, IntoElement, ParentElement as _, div, prelude::*};

use crate::ids::EventId;
use crate::session::{self, SessionState};
use crate::telemetry::{self, TelemetryMode};
use crate::{connectivity, desktop_actions, secure_storage};

mod desktop_actions_section;
mod event_section;
mod outline;
mod runtime_boundaries_section;
mod section;
mod telemetry_section;
mod toast;

use desktop_actions_section::desktop_actions_section;
use event_section::event_section;
use outline::outline;
use runtime_boundaries_section::runtime_boundaries_section;
pub use section::section;
use telemetry_section::telemetry_section;
use toast::toast;
