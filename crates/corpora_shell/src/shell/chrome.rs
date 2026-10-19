use ely_gpui_component::{
    buttons::{ButtonVariant, IconButton},
    layout::Sidebar,
    motion::Skeleton,
    primitives::IconName,
    shell::{ActivityBar, ActivityItem, TabBar, TitleBar, WindowTab},
    typography::{Caption, Overline},
};
use gpui::{App, IntoElement, WeakEntity, Window, div, prelude::*};
use gpui_router::{Outlet, use_location, use_navigate};

use super::Workspace;
use crate::{navigation, pages, routes};

mod frame;
mod sidebar;

pub use frame::frame;
use sidebar::sidebar;
