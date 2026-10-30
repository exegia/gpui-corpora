use gpui::{
    AnyElement, App, ClickEvent, Context, Div, ElementId, IntoElement, ParentElement, RenderOnce,
    SharedString, StyleRefinement, Styled, Window, div, prelude::*,
};
use gpui_kit::component::popover::PopoverState;

use super::{BreadcrumbPage, BreadcrumbTrigger};

type Content =
    Box<dyn Fn(&mut PopoverState, &mut Window, &mut Context<PopoverState>) -> AnyElement>;
type Menu = Box<dyn FnOnce(BreadcrumbTrigger) -> AnyElement>;

/// The four item contracts supported by the composed breadcrumb.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BreadcrumbVariant {
    Default,
    Link,
    Toc,
    Menu,
}

enum Kind {
    Default(AnyElement),
    Link(Box<BreadcrumbLink>),
    Toc(Content),
    Menu(Menu),
}

/// One breadcrumb item. Constructors require the data belonging to each variant.
#[derive(IntoElement)]
pub struct BreadcrumbItem {
    base: Div,
    trigger: BreadcrumbTrigger,
    id: ElementId,
    kind: Kind,
}

impl BreadcrumbItem {
    fn build(id: ElementId, label: SharedString, kind: Kind) -> Self {
        Self {
            base: div().flex().items_center().gap_1p5(),
            trigger: BreadcrumbTrigger::new(id.clone(), label),
            id,
            kind,
        }
    }

    /// Renders noninteractive content in a disabled ghost button.
    pub fn new(
        id: impl Into<ElementId>,
        label: impl Into<SharedString>,
        content: impl IntoElement,
    ) -> Self {
        Self::build(
            id.into(),
            label.into(),
            Kind::Default(content.into_any_element()),
        )
    }

    pub fn page(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
        let label = label.into();
        Self::new(id, label.clone(), BreadcrumbPage::new(label))
    }

    /// Opens a URL by default; `on_click` can supply native in-app navigation instead.
    pub fn link(
        id: impl Into<ElementId>,
        label: impl Into<SharedString>,
        href: impl Into<SharedString>,
    ) -> Self {
        let (id, label) = (id.into(), label.into());
        Self::build(
            id.clone(),
            label.clone(),
            Kind::Link(Box::new(BreadcrumbLink::new(id, label, href))),
        )
    }

    /// Owns the popover; the caller supplies its already-configured content.
    pub fn toc<E: IntoElement>(
        id: impl Into<ElementId>,
        label: impl Into<SharedString>,
        content: impl Fn(&mut PopoverState, &mut Window, &mut Context<PopoverState>) -> E + 'static,
    ) -> Self {
        Self::build(
            id.into(),
            label.into(),
            Kind::Toc(Box::new(move |state, window, cx| {
                content(state, window, cx).into_any_element()
            })),
        )
    }

    /// The supplied component receives the trigger and owns its menu and data.
    pub fn menu<E: IntoElement>(
        id: impl Into<ElementId>,
        label: impl Into<SharedString>,
        component: impl FnOnce(BreadcrumbTrigger) -> E + 'static,
    ) -> Self {
        Self::build(
            id.into(),
            label.into(),
            Kind::Menu(Box::new(move |trigger| {
                component(trigger).into_any_element()
            })),
        )
    }

    pub fn variant(&self) -> BreadcrumbVariant {
        match self.kind {
            Kind::Default(_) => BreadcrumbVariant::Default,
            Kind::Link(_) => BreadcrumbVariant::Link,
            Kind::Toc(_) => BreadcrumbVariant::Toc,
            Kind::Menu(_) => BreadcrumbVariant::Menu,
        }
    }

    pub fn tooltip(mut self, tooltip: impl Into<SharedString>) -> Self {
        let tooltip = tooltip.into();
        if let Kind::Link(link) = &mut self.kind {
            link.tooltip = Some(tooltip.clone());
        }
        self.trigger = self.trigger.tooltip(tooltip);
        self
    }

    /// A trigger callback, separate from any menu-option selection callback.
    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        match &mut self.kind {
            Kind::Link(link) => link.on_click = Some(Box::new(handler)),
            Kind::Menu(_) => self.trigger = self.trigger.on_click(handler),
            _ => {}
        }
        self
    }
}

impl Styled for BreadcrumbItem {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl RenderOnce for BreadcrumbItem {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let content = match self.kind {
            Kind::Default(content) => self
                .trigger
                .content(content)
                .disabled(true)
                .into_any_element(),
            Kind::Link(link) => (*link).into_any_element(),
            Kind::Toc(content) => {
                super::BreadcrumbPopup::new(self.id.clone(), self.trigger, content)
                    .into_any_element()
            }
            Kind::Menu(component) => component(self.trigger.menu()),
        };
        self.base
            .id((self.id, "item"))
            .role(gpui::Role::ListItem)
            .child(content)
    }
}

/// A low-level breadcrumb destination, with a native navigation override.
#[derive(IntoElement)]
pub struct BreadcrumbLink {
    trigger: BreadcrumbTrigger,
    href: SharedString,
    tooltip: Option<SharedString>,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App)>>,
}

impl BreadcrumbLink {
    pub fn new(
        id: impl Into<ElementId>,
        label: impl Into<SharedString>,
        href: impl Into<SharedString>,
    ) -> Self {
        Self {
            trigger: BreadcrumbTrigger::new(id, label),
            href: href.into(),
            tooltip: None,
            on_click: None,
        }
    }

    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }
}

impl RenderOnce for BreadcrumbLink {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        self.trigger
            .when_some(self.tooltip, |trigger, tooltip| trigger.tooltip(tooltip))
            .on_click(move |event, window, cx| {
                if let Some(handler) = &self.on_click {
                    handler(event, window, cx);
                } else {
                    cx.open_url(&self.href);
                }
            })
    }
}
