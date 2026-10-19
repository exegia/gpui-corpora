use ely_gpui_component::theme::ActiveTheme;
use ely_gpui_component::{
    buttons::{ButtonVariant, IconButton},
    layout::Sidebar,
    motion::Skeleton,
    primitives::IconName,
    shell::{ActivityBar, ActivityItem, TabBar, TitleBar, WindowTab},
    typography::{Caption, Overline},
};
use gpui::{App, IntoElement, Rems, StyleRefinement, WeakEntity, Window, div, prelude::*, rems};
use gpui_router::{Outlet, use_location, use_navigate};

use super::Workspace;
use crate::{navigation, pages, routes};

mod frame;
mod sidebar;
mod style_chrome;

pub use frame::frame;
use sidebar::sidebar;
pub(super) use style_chrome::style_chrome;

pub(crate) const TITLEBAR_PADDING: Rems = rems(0.3);
