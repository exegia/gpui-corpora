#[allow(unused_imports)]
use super::*;

/// Fallback view shown by the error boundary when a page render panics.
pub struct RenderErrorPage {
    pub(crate) summary: SharedString,
}

impl RenderErrorPage {
    pub fn new(summary: String) -> Self {
        Self {
            summary: summary.into(),
        }
    }
}

impl Render for RenderErrorPage {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        ErrorView::new(
            "render-error",
            IconName::CircleAlert,
            crate::i18n::localize("render_error_title"),
        )
        .body(self.summary.clone())
        .action(
            Button::new(
                "reload-current-page",
                crate::i18n::localize("render_error_reload"),
            )
            .primary()
            .on_click(|_, window, cx| {
                // Window-scoped: App::dispatch_action is broken on
                // Windows (thread-local active-window lookup).
                window.dispatch_action(Box::new(ReloadCurrentPage), cx);
            }),
        )
    }
}
