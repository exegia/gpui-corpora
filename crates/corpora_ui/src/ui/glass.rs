use crate::ely::theme::ActiveTheme;

use gpui::{Corners, Hsla, IntoElement, Pixels, Styled, canvas, px};

#[derive(Clone, Copy)]
struct GlassProfile {
    magnify: f32,
    dispersion: f32,
    edge: f32,
    edge_width: Pixels,
}

const STANDARD: GlassProfile = GlassProfile {
    magnify: 1.2,
    dispersion: 0.014,
    edge: 0.26,
    edge_width: px(1.4),
};

const DEEP_BEZEL: GlassProfile = GlassProfile {
    magnify: 1.38,
    dispersion: 0.02,
    edge: 0.34,
    edge_width: px(2.),
};

/// Clear glass tinted by the active theme's highlight color.
pub fn clear(radius: Pixels) -> impl IntoElement {
    surface(radius, None, STANDARD)
}

/// Clear glass with stronger refraction and a deeper illuminated rim.
pub(crate) fn clear_deep(radius: Pixels) -> impl IntoElement {
    surface(radius, None, DEEP_BEZEL)
}

/// Glass with a translucent semantic tint supplied by the caller.
pub fn tinted(radius: Pixels, tint: Hsla) -> impl IntoElement {
    surface(radius, Some(tint), STANDARD)
}

fn surface(radius: Pixels, tint: Option<Hsla>, profile: GlassProfile) -> impl IntoElement {
    canvas(
        |_, _, _| (),
        move |bounds, _, window, cx| {
            let dark = cx.theme().is_dark();
            let corners = Corners::all(radius);
            let tint = tint.unwrap_or_else(|| {
                cx.theme()
                    .colors
                    .on_media
                    .opacity(if dark { 16. / 255. } else { 18.8 / 255. })
            });
            #[cfg(target_os = "macos")]
            window.paint_layer(bounds, |window| {
                let half_extent = bounds.size.width.min(bounds.size.height) / 2.;
                window.paint_backdrop_blur(
                    bounds,
                    corners,
                    gpui::GlassEffect {
                        blur_radius: px(1.2),
                        lens: px(28.).min(half_extent),
                        reach: px(64.).min(half_extent),
                        magnify: profile.magnify,
                        dispersion: profile.dispersion,
                        gain: if dark { 1.029 } else { 1.041 },
                        saturation: 1.,
                        tint,
                        edge: profile.edge,
                        edge_width: profile.edge_width,
                        edge_aa: px(0.5),
                    },
                );
            });
            #[cfg(not(target_os = "macos"))]
            window.paint_quad(gpui::fill(bounds, tint).corner_radii(corners));
        },
    )
    .size_full()
}
