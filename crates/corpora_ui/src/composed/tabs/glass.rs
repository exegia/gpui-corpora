use crate::ely::theme::ActiveTheme;

use gpui::{Corners, IntoElement, Styled, canvas, px};

pub(super) fn clear(dark: bool) -> impl IntoElement {
    canvas(
        |_, _, _| (),
        move |bounds, _, window, cx| {
            let corners = Corners::all(px(14.));
            let tint =
                cx.theme()
                    .colors
                    .on_media
                    .opacity(if dark { 16. / 255. } else { 18.8 / 255. });
            #[cfg(target_os = "macos")]
            window.paint_layer(bounds, |window| {
                let half_extent = bounds.size.width.min(bounds.size.height) / 2.;
                window.paint_backdrop_blur(
                    bounds,
                    corners,
                    gpui::GlassEffect {
                        blur_radius: px(1.2),
                        lens: px(18.75).min(half_extent),
                        reach: px(47.).min(half_extent),
                        magnify: 1.1,
                        dispersion: 0.005,
                        gain: if dark { 1.029 } else { 1.041 },
                        saturation: 1.,
                        tint,
                        edge: 0.26,
                        edge_width: px(1.4),
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
