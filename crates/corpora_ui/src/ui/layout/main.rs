use gpui::{Div, IntoElement, div, prelude::*};

use super::window_background;

/// A clipped workspace surface using the window's background treatment.
pub fn main(dark: bool, content: impl IntoElement) -> Div {
    div()
        .relative()
        .size_full()
        .overflow_hidden()

        .child(div().relative().size_full().child(content))
}
