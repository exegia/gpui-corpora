use std::rc::Rc;

use gpui::{AnyElement, App, ElementId, IntoElement, SharedString, StyleRefinement, Window};

pub(super) type TabHandler = Rc<dyn Fn(&usize, &mut Window, &mut App)>;

/// Visual layout for a tab list.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TabVariant {
    /// Icon above label, sized for settings and compact navigation.
    #[default]
    Vertical,
    /// Leading icon, full label, and optional close control.
    Horizontal,
}

/// A tab's label, optional icon, and close capability. The label is the tab's
/// accessible name; the icon is decorative.
pub struct Tab {
    pub(super) label: SharedString,
    pub(super) icon: Option<AnyElement>,
    pub(super) closable: bool,
}

impl Tab {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            icon: None,
            closable: false,
        }
    }

    pub fn icon(mut self, icon: impl IntoElement) -> Self {
        self.icon = Some(icon.into_any_element());
        self
    }

    pub fn closable(mut self, closable: bool) -> Self {
        self.closable = closable;
        self
    }
}

/// A controlled tab list. Callers retain selection and render panel content.
#[derive(IntoElement)]
pub struct Tabs {
    pub(super) id: ElementId,
    pub(super) label: SharedString,
    pub(super) items: Vec<Tab>,
    pub(super) selected: Option<usize>,
    pub(super) autofocus: bool,
    pub(super) on_change: Option<TabHandler>,
    pub(super) on_close: Option<TabHandler>,
    pub(super) style: StyleRefinement,
    pub(super) variant: TabVariant,
}
