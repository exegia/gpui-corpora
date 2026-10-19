//! Application entry point, kept in the lib so `src/main.rs` stays a thin shim.

use std::time::Duration;

use ely_gpui_component::{
    Assets,
    primitives::IconName,
    shell::{SplashScreen, TrayIcon, TrayItem, set_dock_badge},
};
use gpui::App;

use crate::navigation;

mod bootstrap;
mod open_with_splash;
mod tray;
mod tray_paths;
mod tray_quit;

pub use bootstrap::bootstrap;
use open_with_splash::open_with_splash;
use tray::tray;
use tray_paths::TRAY_PATHS;
use tray_quit::TRAY_QUIT;
