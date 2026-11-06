//! Explicit breadcrumb parts. Callers own navigation, menu data, and TOC content.

use gpui::{
    App, Div, IntoElement, ParentElement, RenderOnce, SharedString, StyleRefinement, Styled,
    Window, div, prelude::*,
};

use crate::ely::{
    primitives::{Icon, IconName},
    theme::{ActiveTheme, IconSize},
};

mod item;
mod popup;
mod separator;
mod trigger;

pub use item::{BreadcrumbItem, BreadcrumbLink, BreadcrumbVariant};
pub use popup::BreadcrumbPopup;
pub use separator::{BreadcrumbSeparator, BreadcrumbSeparatorSymbol};
pub use trigger::BreadcrumbTrigger;

/// Navigation landmark containing one breadcrumb list.
#[derive(IntoElement)]
pub struct Breadcrumb {
    base: Div,
    id: gpui::ElementId,
    label: SharedString,
}

impl Breadcrumb {
    pub fn new(id: impl Into<gpui::ElementId>) -> Self {
        Self {
            base: div(),
            id: id.into(),
            label: "Breadcrumb".into(),
        }
    }

    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = label.into();
        self
    }
}

impl ParentElement for Breadcrumb {
    fn extend(&mut self, elements: impl IntoIterator<Item = gpui::AnyElement>) {
        self.base.extend(elements);
    }
}

impl Styled for Breadcrumb {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl RenderOnce for Breadcrumb {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        self.base
            .id(self.id)
            .role(gpui::Role::Navigation)
            .aria_label(self.label)
    }
}

/// A wrapping list; separators are explicit children rather than inferred gaps.
#[derive(IntoElement)]
pub struct BreadcrumbList {
    base: Div,
}

impl BreadcrumbList {
    pub fn new() -> Self {
        Self {
            base: div().flex().flex_wrap().items_center().min_w_0().text_sm(),
        }
    }
}

impl Default for BreadcrumbList {
    fn default() -> Self {
        Self::new()
    }
}

impl ParentElement for BreadcrumbList {
    fn extend(&mut self, elements: impl IntoIterator<Item = gpui::AnyElement>) {
        self.base.extend(elements);
    }
}

impl Styled for BreadcrumbList {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl RenderOnce for BreadcrumbList {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        self.base
            .id("breadcrumb-list")
            .role(gpui::Role::List)
            .text_color(cx.theme().colors.fg_muted)
    }
}

/// Current-location text, normally inside a disabled default item.
#[derive(IntoElement)]
pub struct BreadcrumbPage {
    base: Div,
}

impl BreadcrumbPage {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            base: div()
                .font_weight(gpui::FontWeight::NORMAL)
                .child(label.into()),
        }
    }
}

impl Styled for BreadcrumbPage {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl RenderOnce for BreadcrumbPage {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        self.base
            .id("current-page")
            .aria_description("Current page")
            .text_color(cx.theme().colors.fg)
    }
}

/// A decorative ellipsis. It neither collapses items nor opens a menu.
#[derive(IntoElement)]
pub struct BreadcrumbEllipsis;

impl RenderOnce for BreadcrumbEllipsis {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        div().flex().items_center().justify_center().size_4().child(
            Icon::new(IconName::Ellipsis)
                .size(IconSize::Sm)
                .color(cx.theme().colors.fg_muted),
        )
    }
}

#[cfg(test)]
mod tests;
