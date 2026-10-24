#[allow(unused_imports)]
use super::*;

pub(crate) fn page(
    title: impl Into<gpui::SharedString>,
    description: impl Into<gpui::SharedString>,
) -> Div {
    div()
        .flex()
        .flex_col()
        .gap_6()
        .w_full()
        .child(crate::shell::dev_target(
            "page-heading",
            "Page title and description",
            div()
                .flex()
                .flex_col()
                .gap_2()
                .child(Heading::h1(title))
                .child(Paragraph::new(description)),
        ))
}
