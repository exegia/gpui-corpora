#[allow(unused_imports)]
use super::*;
use std::borrow::Cow;

use gpui::{Svg, svg};

/// The Corpora mark as one monochrome SVG. It fills with `currentColor`, so
/// callers tint it with `text_color` and it reads on any surface.
pub const LOGO_SVG: &[u8] = include_bytes!("../assets/brand/corpora.svg");

pub fn logo() -> Svg {
    svg().data(LOGO_SVG)
}

/// Register the bundled Caudex regular and bold fonts before opening windows.
pub fn register_brand_fonts(cx: &App) -> anyhow::Result<()> {
    cx.text_system().add_fonts(vec![
        Cow::Borrowed(include_bytes!("../assets/fonts/caudex/Caudex-Regular.ttf").as_slice()),
        Cow::Borrowed(include_bytes!("../assets/fonts/caudex/Caudex-Bold.ttf").as_slice()),
    ])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn logo_svg_rasterizes_to_a_visible_mark() {
        let image = gpui::Image::from_bytes(gpui::ImageFormat::Svg, LOGO_SVG.to_vec())
            .to_image_data(gpui::SvgRenderer::new(std::sync::Arc::new(())))
            .expect("logo svg parses and renders");
        let size = image.size(0);
        let (w, h) = (size.width.0 as usize, size.height.0 as usize);
        assert_eq!(w, h, "the mark is square");
        let bytes = image.as_bytes(0).expect("rendered frame");
        let alpha = |x: usize, y: usize| bytes[(y * w + x) * 4 + 3];
        // The ring is opaque; the corner outside the circle is transparent.
        assert_eq!(alpha(w * 3 / 100, h / 2), 0xFF);
        assert_eq!(alpha(0, 0), 0);
        let visible = bytes.as_chunks::<4>().0.iter().filter(|p| p[3] > 0).count();
        assert!(visible > w * h / 2, "disc covers most of the canvas");
    }
}
