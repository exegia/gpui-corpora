use corpora_ui::ely::primitives::IconName;
use gpui::{App, SharedString};
use serde::{Deserialize, Serialize};

use crate::{
    app::theme::ThemeMode,
    events::{self, AppEventKind},
    navigation,
};

mod availability;
mod command;
mod command_availability;
mod command_id;
mod command_spec;
mod disabled_reason;
mod execute;
mod navigate;
mod registry;
mod report_error;

pub use availability::availability;
use command::command;
pub use command_availability::CommandAvailability;
pub use command_id::CommandId;
pub use command_spec::CommandSpec;
use disabled_reason::disabled_reason;
pub use execute::execute;
use navigate::navigate;
pub use registry::registry;
use report_error::report_error;
