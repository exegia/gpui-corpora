use ely_gpui_component::{
    buttons::Button,
    feedback::EmptyState,
    layout::{Card, CardHeader},
    primitives::IconName,
    project::{Priority, PriorityIndicator, Status, StatusSelect},
    typography::{Caption, Heading, Overline, Paragraph},
};
use gpui::{App, Div, WeakEntity, div, prelude::*};
use gpui_router::{use_navigate, use_params};

use crate::shell::Workspace;

mod authentication;
mod empty;
mod explore_detail;
mod home;
mod information;
mod license;
mod license_create;
mod licenses;
mod nav_button;
mod not_found;
mod page;
mod parameter;
mod project;
mod projects;
mod search;
mod statuses;
mod viewer;

pub use authentication::authentication;
use empty::empty;
pub use explore_detail::explore_detail;
pub use home::home;
pub use information::information;
pub use license::license;
pub use license_create::license_create;
pub use licenses::licenses;
pub use nav_button::nav_button;
pub use not_found::not_found;
use page::page;
use parameter::parameter;
pub use project::project;
pub use projects::projects;
pub use search::search;
pub use statuses::statuses;
pub use viewer::viewer;
