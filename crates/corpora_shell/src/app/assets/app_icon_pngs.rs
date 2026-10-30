#[allow(unused_imports)]
use super::*;

/// Branded app-icon PNGs for surfaces with no icon-theme fallback (the X11
/// window icon).
#[cfg(target_os = "linux")]
#[derive(rust_embed::RustEmbed)]
#[folder = "src/shell/assets/app-icon/hicolor"]
pub(crate) struct AppIconPngs;
