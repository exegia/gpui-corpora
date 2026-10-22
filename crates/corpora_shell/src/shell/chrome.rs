use corpora_ui::ely::theme::ActiveTheme;
use corpora_ui::ely::{
    buttons::{Button, ButtonVariant, IconButton},
    motion::Skeleton,
    primitives::IconName,
    shell::{ActivityBar, ActivityItem, TitleBar},
};
use gpui::{App, IntoElement, Rems, StyleRefinement, WeakEntity, Window, div, prelude::*, rems};
use gpui_router::{Outlet, use_location, use_navigate};

use super::{EMPTY_TAB_PREFIX, Workspace};
use crate::{navigation, routes};

mod brand;
mod frame;
mod notifications;
mod panels;
mod style_chrome;
mod tabs;

pub use brand::{logo, logo_image, register_brand_fonts};
pub use frame::frame;
pub(super) use style_chrome::style_chrome;

pub(crate) const TITLEBAR_PADDING: Rems = rems(0.3);
