#[allow(unused_imports)]
use super::*;

/// A wrapping row of buttons under a settings section.
pub(crate) fn button_row(buttons: impl IntoIterator<Item = Button>) -> Div {
    div()
        .flex()
        .flex_wrap()
        .items_center()
        .gap_2()
        .pt_3()
        .children(buttons)
}
