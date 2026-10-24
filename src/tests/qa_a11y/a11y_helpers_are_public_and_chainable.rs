#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn a11y_helpers_are_public_and_chainable() {
    let _element = div()
        .id("save")
        .a11y(Role::Button, "Save")
        .aria_description("Stores the current form")
        .a11y_live(Live::Polite);
}
