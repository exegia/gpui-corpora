#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn element_helpers_chain_on_stateful_divs() {
    // Compile-shape guard: the helpers resolve on Stateful<Div> and stay
    // chainable with gpui's own aria builders.
    let _element = div()
        .id("save")
        .a11y(Role::Button, "Save")
        .aria_description("Stores the current form")
        .a11y_live(Live::Polite);
}
