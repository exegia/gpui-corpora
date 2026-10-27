use crate::ely::theme::ActiveTheme;

use gpui::{App, Bounds, Corners, Hsla, IntoElement, Pixels, Styled, Window, canvas, px};

/// Tunable parameters for the backdrop blur and lens painted by a glass surface.
#[derive(Clone, Copy, Debug)]
pub struct GlassSpec {
    pub blur_radius: Pixels,
    pub lens: Pixels,
    pub reach: Pixels,
    pub magnify: f32,
    pub dispersion: f32,
    pub gain_light: f32,
    pub gain_dark: f32,
    pub saturation: f32,
    pub tint: Option<Hsla>,
    pub edge: f32,
    pub edge_width: Pixels,
    pub edge_aa: Pixels,
}

impl GlassSpec {
    fn deep() -> Self {
        Self {
            magnify: 1.38,
            dispersion: 0.02,
            edge: 0.34,
            edge_width: px(2.),
            ..Self::default()
        }
    }
}

impl Default for GlassSpec {
    fn default() -> Self {
        Self {
            blur_radius: px(1.2),
            lens: px(28.),
            reach: px(64.),
            magnify: 1.2,
            dispersion: 0.014,
            gain_light: 1.041,
            gain_dark: 1.029,
            saturation: 1.,
            tint: None,
            edge: 0.26,
            edge_width: px(1.4),
            edge_aa: px(0.5),
        }
    }
}

/// Clear glass tinted by the active theme's highlight color.
pub fn clear(radius: Pixels) -> impl IntoElement {
    surface(radius, GlassSpec::default())
}

/// Clear glass with stronger refraction and a deeper illuminated rim.
pub(crate) fn clear_deep(radius: Pixels) -> impl IntoElement {
    surface(radius, GlassSpec::deep())
}

/// Glass with a translucent semantic tint supplied by the caller.
pub fn tinted(radius: Pixels, tint: Hsla) -> impl IntoElement {
    surface(
        radius,
        GlassSpec {
            tint: Some(tint),
            ..GlassSpec::default()
        },
    )
}

fn surface(radius: Pixels, spec: GlassSpec) -> impl IntoElement {
    canvas(
        |_, _, _| (),
        move |bounds, _, window, cx| {
            #[cfg(target_os = "macos")]
            window.paint_layer(bounds, |window| {
                paint(bounds, Corners::all(radius), spec, window, cx);
            });
            #[cfg(not(target_os = "macos"))]
            paint(bounds, Corners::all(radius), spec, window, cx);
        },
    )
    .size_full()
}

pub(super) fn paint(
    bounds: Bounds<Pixels>,
    corners: Corners<Pixels>,
    spec: GlassSpec,
    window: &mut Window,
    cx: &mut App,
) {
    let dark = cx.theme().is_dark();
    let tint = spec.tint.unwrap_or_else(|| {
        cx.theme()
            .colors
            .on_media
            .opacity(if dark { 16. / 255. } else { 18.8 / 255. })
    });

    #[cfg(target_os = "macos")]
    {
        let half_extent = bounds.size.width.min(bounds.size.height) / 2.;
        window.paint_backdrop_blur(
            bounds,
            corners,
            gpui::GlassEffect {
                blur_radius: spec.blur_radius,
                lens: spec.lens.min(half_extent),
                reach: spec.reach.min(half_extent),
                magnify: spec.magnify,
                dispersion: spec.dispersion,
                gain: if dark {
                    spec.gain_dark
                } else {
                    spec.gain_light
                },
                saturation: spec.saturation,
                tint,
                edge: spec.edge,
                edge_width: spec.edge_width,
                edge_aa: spec.edge_aa,
            },
        );
    }

    #[cfg(not(target_os = "macos"))]
    window.paint_quad(gpui::fill(bounds, tint).corner_radii(corners));
}
