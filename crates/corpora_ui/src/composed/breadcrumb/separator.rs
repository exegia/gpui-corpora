use gpui::{
    AnyElement, App, ElementId, IntoElement, RenderOnce, SharedString, Window, div, prelude::*,
};

use super::BreadcrumbTrigger;
use crate::ely::{
    primitives::{Icon, IconName},
    theme::{ActiveTheme, IconSize},
};

/// Built-in decorative symbols; custom content can override either one.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum BreadcrumbSeparatorSymbol {
    #[default]
    Chevron,
    Slash,
}

type Menu = Box<dyn FnOnce(BreadcrumbTrigger) -> AnyElement>;

/// A decorative symbol, or an accessible icon-sized trigger owned by a menu.
#[derive(IntoElement)]
pub struct BreadcrumbSeparator {
    symbol: BreadcrumbSeparatorSymbol,
    content: Option<AnyElement>,
    menu: Option<(ElementId, SharedString, Menu)>,
    disabled: bool,
}

impl BreadcrumbSeparator {
    pub fn new() -> Self {
        Self {
            symbol: BreadcrumbSeparatorSymbol::Chevron,
            content: None,
            menu: None,
            disabled: false,
        }
    }

    pub fn symbol(mut self, symbol: BreadcrumbSeparatorSymbol) -> Self {
        self.symbol = symbol;
        self
    }

    pub fn content(mut self, content: impl IntoElement) -> Self {
        self.content = Some(content.into_any_element());
        self
    }

    pub fn menu<E: IntoElement>(
        id: impl Into<ElementId>,
        label: impl Into<SharedString>,
        component: impl FnOnce(BreadcrumbTrigger) -> E + 'static,
    ) -> Self {
        Self {
            menu: Some((
                id.into(),
                label.into(),
                Box::new(move |trigger| component(trigger).into_any_element()),
            )),
            ..Self::new()
        }
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl Default for BreadcrumbSeparator {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for BreadcrumbSeparator {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let content = self.content.unwrap_or_else(|| match self.symbol {
            BreadcrumbSeparatorSymbol::Chevron => Icon::new(IconName::ChevronRight)
                .size(IconSize::Sm)
                .color(cx.theme().colors.fg_muted)
                .into_any_element(),
            BreadcrumbSeparatorSymbol::Slash => div().child("/").into_any_element(),
        });
        let content = if let Some((id, label, component)) = self.menu {
            let trigger = BreadcrumbTrigger::new(id, label.clone())
                .tooltip(label)
                .content(content)
                .icon_only()
                .disabled(self.disabled);
            if self.disabled {
                trigger.into_any_element()
            } else {
                component(trigger)
            }
        } else {
            content
        };
        div()
            .flex()
            .items_center()
            .flex_none()
            .opacity(0.8)
            .child(content)
    }
}
