mod rows;

use ely_gpui_component::{
    buttons::{Button, ButtonVariant},
    data_display::DescriptionList,
    typography::Heading,
};
use gpui::{prelude::*, *};

use crate::{
    accessibility, app_state, capabilities, crash_report, desktop_actions, error_surface,
    lifecycle::LifecycleState, notifications, shortcuts, storage, telemetry, undo_stack,
};

mod capability_row;
mod diagnostics_page;
mod row;

use capability_row::capability_row;
pub use diagnostics_page::DiagnosticsPage;
use row::row;
