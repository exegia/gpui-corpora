use gpui::{AnyElement, App, Div, RenderOnce, Stateful, StyleRefinement, Window, prelude::*};

// Ely's pinned shell roots apply their borders and sizes after builder styles.
pub(in crate::shell) fn style_chrome(
    component: impl RenderOnce,
    style: StyleRefinement,
    window: &mut Window,
    cx: &mut App,
) -> AnyElement {
    let mut element = component.render(window, cx).into_any_element();
    if let Some(root) = element.downcast_mut::<Stateful<Div>>() {
        root.style().refine(&style);
    } else {
        element
            .downcast_mut::<Div>()
            .expect("pinned Ely shell component must render a Div")
            .style()
            .refine(&style);
    }
    element
}
