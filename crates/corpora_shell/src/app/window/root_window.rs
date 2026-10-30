#[allow(unused_imports)]
use super::*;

/// The main app window, recorded at creation so code off the OS-input path
/// (tray, shutdown) can reach it without `active_window()`, which resolves
/// via thread-local `GetActiveWindow()` on Windows.
#[derive(Clone, Copy)]
pub struct RootWindow(pub AnyWindowHandle);

impl Global for RootWindow {}

/// The recorded main window, if one has been opened.
pub fn root_window(cx: &App) -> Option<AnyWindowHandle> {
    cx.try_global::<RootWindow>().map(|root| root.0)
}
