use gpui::{Div, div, linear_color_stop, linear_gradient, prelude::*, rgb};

/// A theme tint over the native window material, filling its relative parent.
pub fn window_background(dark: bool) -> Div {
    let (start, end) = if dark {
        (rgb(0x181818), rgb(0x202020))
    } else {
        (rgb(0xf1f5f9), rgb(0xfafafa))
    };
    div().absolute().inset_0().bg(linear_gradient(
        227.663,
        linear_color_stop(start, 0.30937),
        linear_color_stop(end.opacity(0.27), 0.98074),
    ))
}
