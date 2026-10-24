use ely_gpui_component::{primitives::IconName, shell::AboutDialog};
use gpui::{prelude::*, *};

pub struct AboutPage;

impl AboutPage {
    pub fn new() -> Self {
        Self
    }
}

impl Default for AboutPage {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for AboutPage {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div().flex().justify_center().pt_8().child(
            // Version from Cargo.toml; a locale string here would drift out of sync.
            AboutDialog::new("Corpora", env!("CARGO_PKG_VERSION"))
                .icon(IconName::BookOpen)
                .description(crate::i18n::localize("about_tagline")),
        )
    }
}
