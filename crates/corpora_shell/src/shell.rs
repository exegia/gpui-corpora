//! The workspace window: chrome, routed pages, command palette, toasts, and
//! the app event queue that drives navigation.

mod chrome;
mod dev_feedback;
mod frame_time;
mod status;
pub use dev_feedback::Devtools;
pub(crate) use dev_feedback::target as dev_target;

use std::{collections::HashMap, time::Duration};

#[cfg(not(debug_assertions))]
use corpora_ui::ely::primitives::FocusScope;
use corpora_ui::ely::{
    feedback::{EmptyState, ToastViewport, Toaster},
    layout::{Dock, DockPanel, DockSide},
    navigation::{Command, CommandPalette},
    primitives::IconName,
    project::Status,
    shell::TrayIcon,
    theme::{ActiveTheme, TextSize},
};
use gpui::{
    Action, AnyView, App, Context, Entity, FocusHandle, Global, KeyBinding, Subscription,
    WeakEntity, Window, div, prelude::*,
};
use gpui_router::use_navigate;

use crate::{
    events::{self, AppEventKind},
    navigation, routes,
    views::{
        AboutPage, DiagnosticsPage, NotificationsPage, ReloadCurrentPage, RenderErrorPage,
        TriggerRenderError,
    },
};

pub(crate) use chrome::TITLEBAR_PADDING;
pub use chrome::frame;
pub use chrome::{logo, logo_image, register_brand_fonts};
pub use frame_time::last_frame_time_us;

mod command_value;
mod navigate_to_section;
mod notify_on;
mod open_palette;
mod pages;
mod pane_visibility;
mod tab_workspace;
mod workspace;
mod workspace_handle;
mod workspace_tabs;

use command_value::command_value;
use corpora_ui::ui::layout::window_background;
pub use navigate_to_section::NavigateToSection;
use notify_on::notify_on;
pub use open_palette::open_palette;
pub use pages::Pages;
pub use workspace::Workspace;
use workspace_handle::WorkspaceHandle;
pub(crate) use workspace_handle::current_workspace;
use workspace_tabs::{EMPTY_TAB_PREFIX, WorkspaceTabs};
