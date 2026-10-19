//! The workspace window: chrome, routed pages, command palette, toasts, and
//! the app event queue that drives navigation.

mod chrome;
mod frame_time;
mod status;

use std::{collections::HashMap, time::Duration};

use ely_gpui_component::{
    feedback::{EmptyState, ToastViewport, Toaster},
    layout::{Dock, DockPanel, DockSide},
    navigation::{Command, CommandPalette},
    primitives::{FocusScope, IconName},
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
        SettingsPage, TriggerRenderError,
    },
};

pub(crate) use chrome::TITLEBAR_PADDING;
pub use chrome::frame;
pub use frame_time::last_frame_time_us;

mod command_value;
mod navigate_to_section;
mod notify_on;
mod open_palette;
mod pages;
mod toggle_sidebar;
mod workspace;
mod workspace_handle;

use command_value::command_value;
pub use navigate_to_section::NavigateToSection;
use notify_on::notify_on;
pub use open_palette::open_palette;
pub use pages::Pages;
pub use toggle_sidebar::ToggleSidebar;
pub use workspace::Workspace;
use workspace_handle::WorkspaceHandle;
