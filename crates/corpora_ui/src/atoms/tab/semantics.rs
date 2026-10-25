use gpui::{AnyElement, IntoElement, SharedString};

/// A tab's label and optional caller-supplied icon. The label is the tab's
/// accessible name; the icon is decorative.
pub struct Tab {
    pub(super) label: SharedString,
    pub(super) icon: Option<AnyElement>,
}

impl Tab {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            icon: None,
        }
    }

    pub fn icon(mut self, icon: impl IntoElement) -> Self {
        self.icon = Some(icon.into_any_element());
        self
    }
}
