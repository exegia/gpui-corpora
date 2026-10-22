use gpui::{AnyElement, App, Div, IntoElement, RenderOnce, Window, prelude::*};

use crate::ely::layout::Well;

/// A workspace pane with the shared fill and inset shadows, rounded on every corner.
pub fn main(dark: bool, content: impl IntoElement) -> Surface {
    Surface {
        dark,
        flush_right: false,
        content: content.into_any_element(),
    }
}

#[derive(IntoElement)]
pub struct Surface {
    dark: bool,
    flush_right: bool,
    content: AnyElement,
}

impl Surface {
    /// Squares the right corners, for a pane that meets the window's right edge.
    pub fn flush_right(mut self, flush: bool) -> Self {
        self.flush_right = flush;
        self
    }
}

impl RenderOnce for Surface {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let mut surface = Well::new()
            .size_full()
            .min_w_0()
            .overflow_hidden()
            .child(self.content)
            .render(window, cx)
            .into_any_element();
        surface
            .downcast_mut::<Div>()
            .expect("Ely Well renders a Div")
            .style()
            .refine(&super::surface_style::style(self.dark, self.flush_right));
        surface
    }
}
