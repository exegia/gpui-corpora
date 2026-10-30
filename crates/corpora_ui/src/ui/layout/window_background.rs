use crate::{ely::theme::Palette, styles::utils::ColorExt};
use gpui::{Div, Hsla, div, linear_color_stop, linear_gradient, prelude::*};

/// A theme tint over the native window material, filling its relative parent.
pub fn window_background(colors: &Palette, dark: bool) -> Div {
    let start = Hsla {
        a: 0.9,
        ..colors.surface.darken(if dark { 0.8 } else { 0.0 })
    };
    let end = Hsla {
        a: 0.0,
        ..colors.bg
    };
    div().absolute().inset_0().bg(linear_gradient(
        -150.,
        linear_color_stop(start, 0.30),
        linear_color_stop(end, 1.0),
    ))
}
