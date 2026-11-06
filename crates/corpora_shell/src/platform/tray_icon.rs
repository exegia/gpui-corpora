//! The menu-bar icon: the Corpora mark rendered from the brand SVG.

use std::{io::Cursor, sync::Arc};

use gpui::{Image, ImageFormat, SvgRenderer};

use crate::shell::LOGO_SVG;

/// The mark as an alpha-only PNG. Menu-bar icons are template images, so the
/// system tints them for light and dark bars and only the alpha channel counts.
pub fn tray_icon_png() -> anyhow::Result<Vec<u8>> {
    let rendered = Image::from_bytes(ImageFormat::Svg, LOGO_SVG.to_vec())
        .to_image_data(SvgRenderer::new(Arc::new(())))?;
    let size = rendered.size(0);
    let bytes = rendered
        .as_bytes(0)
        .ok_or_else(|| anyhow::anyhow!("the logo rendered no frame"))?;
    let pixels = bytes
        .as_chunks::<4>()
        .0
        .iter()
        .flat_map(|pixel| [0, 0, 0, pixel[3]])
        .collect();
    let buffer = image::RgbaImage::from_raw(size.width.0 as u32, size.height.0 as u32, pixels)
        .ok_or_else(|| anyhow::anyhow!("the logo frame has the wrong length"))?;
    let mut png = Cursor::new(Vec::new());
    buffer.write_to(&mut png, image::ImageFormat::Png)?;
    Ok(png.into_inner())
}

/// Replaces the symbol Ely gave the status item with the Corpora mark. Call
/// on the main thread right after creating the tray.
#[cfg(target_os = "macos")]
pub fn apply_tray_icon() -> anyhow::Result<()> {
    use objc2::{
        AnyThread, MainThreadMarker, msg_send,
        rc::Retained,
        runtime::{AnyClass, AnyObject},
    };
    use objc2_app_kit::{NSApplication, NSImage, NSView};
    use objc2_foundation::{NSData, NSSize};

    /// Menu-bar icons are 18pt tall; the PNG carries enough pixels for Retina.
    const POINTS: f64 = 18.;

    fn find_button(view: &NSView, class: &AnyClass) -> Option<Retained<NSView>> {
        for child in view.subviews().iter() {
            let is_button: bool = unsafe { msg_send![&*child, isKindOfClass: class] };
            if is_button {
                return Some(child);
            }
            if let Some(found) = find_button(&child, class) {
                return Some(found);
            }
        }
        None
    }

    let mtm = MainThreadMarker::new()
        .ok_or_else(|| anyhow::anyhow!("the tray icon is set on the main thread"))?;
    let class = AnyClass::get(c"NSStatusBarButton")
        .ok_or_else(|| anyhow::anyhow!("AppKit has no NSStatusBarButton"))?;
    let png = tray_icon_png()?;
    let image = NSImage::initWithData(NSImage::alloc(), &NSData::with_bytes(&png))
        .ok_or_else(|| anyhow::anyhow!("AppKit could not decode the tray icon"))?;
    image.setSize(NSSize::new(POINTS, POINTS));
    image.setTemplate(true);

    let app = NSApplication::sharedApplication(mtm);
    let button = app
        .windows()
        .iter()
        .filter_map(|window| window.contentView())
        .find_map(|view| find_button(&view, class))
        .ok_or_else(|| anyhow::anyhow!("the status item's button was not found"))?;
    let button: &AnyObject = &button;
    unsafe {
        let _: () = msg_send![button, setImage: &*image];
    }
    Ok(())
}

#[cfg(not(target_os = "macos"))]
pub fn apply_tray_icon() -> anyhow::Result<()> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tray_icon_is_a_square_alpha_only_png_of_the_mark() {
        let png = tray_icon_png().expect("tray icon renders");
        let decoded = image::load_from_memory_with_format(&png, image::ImageFormat::Png)
            .expect("valid png")
            .to_rgba8();
        assert_eq!(decoded.width(), decoded.height());
        assert!(decoded.pixels().all(|pixel| pixel.0[..3] == [0, 0, 0]));
        let opaque = decoded.pixels().filter(|pixel| pixel.0[3] == 0xFF).count();
        assert!(opaque as u32 > decoded.width() * decoded.height() / 4);
        assert_eq!(decoded.get_pixel(0, 0).0[3], 0);
    }
}
