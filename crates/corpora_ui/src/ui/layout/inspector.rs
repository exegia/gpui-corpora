use gpui::IntoElement;

/// The tab's inspector surface. Its owner supplies content and width.
pub fn inspector(dark: bool, content: impl IntoElement) -> impl IntoElement {
    // The inspector is always the rightmost pane.
    super::main(dark, content).flush_right(true)
}
