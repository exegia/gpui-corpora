use crate::ely::theme::Mode;
use crate::styles::palette::color;
use gpui::{Div, Hsla, div, linear_color_stop, linear_gradient, prelude::*};

/// A theme tint over the native window material, filling its relative parent.
pub fn window_background(dark: bool) -> Div {
    let mode = if dark { Mode::Dark } else { Mode::Light };
    let start = color("border", mode);
    let end = Hsla {
        a: 0.1,
        ..color("disabled", mode)
    };
    div().absolute().inset_0().bg(linear_gradient(
        -150.,
        linear_color_stop(start, 0.20),
        linear_color_stop(end, 1.0),
    ))
}
