#[allow(unused_imports)]
use super::*;

/// 48px branded PNG for `WindowOptions.icon` (X11 `_NET_WM_ICON`). Wayland
/// resolves the icon from the desktop entry instead.
#[cfg(target_os = "linux")]
pub fn window_icon() -> Option<std::sync::Arc<image::RgbaImage>> {
    app_icon_png(48).map(std::sync::Arc::new)
}
