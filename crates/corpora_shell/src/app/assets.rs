//! App-owned assets. Ely's own icons and fonts load through `corpora_ui::ely::Assets`.

#[cfg(all(test, target_os = "linux"))]
#[path = "assets.test/mod.rs"]
mod assets_test;

mod app_icon_png;
mod app_icon_pngs;
mod window_icon;

#[cfg(target_os = "linux")]
pub use app_icon_png::app_icon_png;
#[cfg(target_os = "linux")]
use app_icon_pngs::AppIconPngs;
#[cfg(target_os = "linux")]
pub use window_icon::window_icon;
