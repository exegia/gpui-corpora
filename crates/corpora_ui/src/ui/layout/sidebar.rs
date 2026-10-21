use gpui::{Div, IntoElement};

/// The tab's sidebar surface. Its owner supplies content and width.
pub fn sidebar(dark: bool, content: impl IntoElement) -> Div {
    super::main(dark, content)
}
