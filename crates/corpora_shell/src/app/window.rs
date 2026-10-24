use gpui::{
    AnyWindowHandle, App, AppContext as _, Bounds, Global, Pixels, Size, TitlebarOptions,
    WindowBounds, WindowHandle, WindowKind, WindowOptions, point, px, size,
};

use crate::shell::Workspace;

#[cfg(test)]
#[path = "window.test/mod.rs"]
mod window_test;

mod dispatch_quit;
mod flush_window_bounds;
mod open_workspace_window;
mod persisted_bounds;
mod root_window;

pub use dispatch_quit::dispatch_quit;
pub use flush_window_bounds::flush_window_bounds;
pub use open_workspace_window::open_workspace_window;
pub use persisted_bounds::persisted_bounds;
pub use root_window::RootWindow;
pub use root_window::root_window;
