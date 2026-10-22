use corpora_ui::ely::theme::ActiveTheme;
use corpora_ui::ely::{
    buttons::{Button, ButtonVariant, IconButton},
    motion::Skeleton,
    primitives::IconName,
    shell::TitleBar,
};
use gpui::{App, IntoElement, Rems, StyleRefinement, WeakEntity, Window, div, prelude::*, rems};
use gpui_router::{Outlet, use_location};

use super::{EMPTY_TAB_PREFIX, Workspace};
use crate::{navigation, routes};

mod activity;
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

/// Title-bar icon buttons are fully round; the new-tab button keeps Ely's radius.
fn round(button: IconButton, window: &mut Window, cx: &mut App) -> gpui::AnyElement {
    style_chrome(
        button,
        StyleRefinement::default().rounded_full(),
        window,
        cx,
    )
}
