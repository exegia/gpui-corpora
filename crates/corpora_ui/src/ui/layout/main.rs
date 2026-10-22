use gpui::{AnyElement, App, Div, IntoElement, RenderOnce, Window, prelude::*};

use crate::ely::layout::Well;

/// A workspace pane with the shared fill and inset shadows.
pub fn main(dark: bool, content: impl IntoElement) -> impl IntoElement {
    Surface {
        dark,
        content: content.into_any_element(),
    }
}

#[derive(IntoElement)]
struct Surface {
    dark: bool,
    content: AnyElement,
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
            .refine(&super::surface_style::style(self.dark));
        surface
    }
}
