#[allow(unused_imports)]
use super::*;

/// Decoded `<size>x<size>` branded app-icon PNG, or None when the size is
/// absent from the embedded set or the bytes fail to decode.
#[cfg(target_os = "linux")]
pub fn app_icon_png(size: u16) -> Option<image::RgbaImage> {
    let png = AppIconPngs::get(&format!("{size}x{size}/apps/corpora.png"))?;
    image::load_from_memory(&png.data)
        .ok()
        .map(|img| img.into_rgba8())
}
