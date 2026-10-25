use crate::ely::theme::Palette;
use gpui::{Div, Hsla, div, linear_color_stop, linear_gradient, prelude::*};

/// A theme tint over the native window material, filling its relative parent.
pub fn window_background(colors: &Palette) -> Div {
    let start = colors.border;
    let end = Hsla {
        a: 0.1,
        ..colors.fg_disabled
    };
    div().absolute().inset_0().bg(linear_gradient(
        -150.,
        linear_color_stop(start, 0.20),
        linear_color_stop(end, 1.0),
    ))
}
