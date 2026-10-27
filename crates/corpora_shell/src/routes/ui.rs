use corpora_ui::{
    composed::breadcrumb::{
        Breadcrumb, BreadcrumbEllipsis, BreadcrumbItem, BreadcrumbList, BreadcrumbPopup,
        BreadcrumbSeparator, BreadcrumbSeparatorSymbol, BreadcrumbTrigger,
    },
    ely::{
        forms::{Input, InputEvent, TextInput},
        primitives::{Icon, IconName},
        theme::{ActiveTheme, IconSize, Radius},
        typography::{Heading, Paragraph},
    },
};
use gpui::{
    App, Context, Entity, Focusable, IntoElement, Render, SharedString, Subscription, WeakEntity,
    Window, div, prelude::*,
};
use gpui_kit::base::{Button, StyledExt};

pub mod button;
pub mod glass;

const CONTENTS: &[&str] = &["Item variants", "Preconfigured components"];
const SECTIONS: &[&str] = &["Usage", "Examples", "API notes"];
const CHAPTERS: &[&str] = &["Genesis 1", "Genesis 2"];

gpui::actions!(
    breadcrumb_picker,
    [Previous, Next, First, Last, Choose, FocusSearch]
);

/// Interactive examples of the composed breadcrumb, retained with the route page.
pub struct Ui {
    current: Entity<TextInput>,
    contents: Entity<Picker>,
    sections: Entity<Picker>,
    chapters: Entity<Picker>,
    selected: SharedString,
    _subscriptions: Vec<Subscription>,
}

impl Ui {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        cx.bind_keys([
            gpui::KeyBinding::new("up", Previous, Some("BreadcrumbPicker > ElyInput")),
            gpui::KeyBinding::new("down", Next, Some("BreadcrumbPicker > ElyInput")),
            gpui::KeyBinding::new("home", First, Some("BreadcrumbPicker > ElyInput")),
            gpui::KeyBinding::new("end", Last, Some("BreadcrumbPicker > ElyInput")),
            gpui::KeyBinding::new("enter", Choose, Some("BreadcrumbPicker > ElyInput")),
            gpui::KeyBinding::new("enter", Choose, Some("BreadcrumbPickerRow")),
            gpui::KeyBinding::new("space", Choose, Some("BreadcrumbPickerRow")),
            gpui::KeyBinding::new("secondary-k", FocusSearch, Some("BreadcrumbPicker")),
        ]);
        let current = cx.new(|cx| {
            let mut input = TextInput::new(window, cx).label("Current page");
            input.set_text("Breadcrumb", cx);
            input
        });
        let owner = cx.entity().downgrade();
        let contents =
            cx.new(|cx| Picker::new("Table of Content", CONTENTS, owner.clone(), window, cx));
        let sections = cx.new(|cx| Picker::new("Sections", SECTIONS, owner.clone(), window, cx));
        let chapters = cx.new(|cx| Picker::new("Chapters", CHAPTERS, owner, window, cx));
        let subscription = cx.observe(&current, |_, _, cx| cx.notify());
        Self {
            current,
            contents,
            sections,
            chapters,
            selected: "No location selected".into(),
            _subscriptions: vec![subscription],
        }
    }

    fn select(&mut self, label: impl Into<SharedString>, cx: &mut Context<Self>) {
        self.selected = label.into();
        cx.notify();
    }

    fn link(&self, id: &'static str, label: &'static str, cx: &Context<Self>) -> BreadcrumbItem {
        let owner = cx.entity().downgrade();
        BreadcrumbItem::link(id, label, format!("/ui#{label}"))
            .tooltip(format!("Go to {label}"))
            .on_click(move |_, _, cx| {
                owner.update(cx, |this, cx| this.select(label, cx)).ok();
            })
    }

    fn toc(id: &'static str, label: &'static str, picker: Entity<Picker>) -> BreadcrumbItem {
        BreadcrumbItem::toc(id, label, move |state, window, cx| {
            if state.focus_handle(cx).is_focused(window) {
                picker.read(cx).query.focus_handle(cx).focus(window, cx);
            }
            picker.clone()
        })
        .tooltip("Browse the table of contents")
    }

    fn menu(id: &'static str, label: &'static str, picker: Entity<Picker>) -> BreadcrumbItem {
        BreadcrumbItem::menu(id, label, move |trigger| picker_popup(id, trigger, picker))
            .tooltip("Choose a section")
    }
}

fn picker_popup(
    id: &'static str,
    trigger: BreadcrumbTrigger,
    picker: Entity<Picker>,
) -> BreadcrumbPopup {
    BreadcrumbPopup::new(id, trigger, move |state, window, cx| {
        if state.focus_handle(cx).is_focused(window) {
            picker.read(cx).query.focus_handle(cx).focus(window, cx);
        }
        picker.clone()
    })
}

fn example(
    title: &'static str,
    detail: &'static str,
    content: impl IntoElement,
    cx: &App,
) -> gpui::Div {
    div()
        .flex()
        .flex_col()
        .gap_3()
        .min_w_0()
        .child(div().text_base().font_semibold().child(title))
        .child(
            div()
                .text_sm()
                .text_color(cx.theme().colors.fg_muted)
                .child(detail),
        )
        .child(
            div()
                .min_w_0()
                .w_full()
                .p_5()
                .rounded(cx.theme().radius(Radius::Lg))
                .border_1()
                .border_color(cx.theme().colors.border)
                .child(content),
        )
}

impl Render for Ui {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let page: SharedString = self.current.read(cx).text().to_string().into();
        let trail = Breadcrumb::new("all-variants")
            .label("Documentation location")
            .child(
                BreadcrumbList::new()
                    .child(self.link("usage", "Usage", cx))
                    .child(BreadcrumbSeparator::new())
                    .child(Self::toc("contents", "Contents", self.contents.clone()))
                    .child(BreadcrumbSeparator::new())
                    .child(Self::menu("sections", "Sections", self.sections.clone()))
                    .child(BreadcrumbSeparator::new())
                    .child(BreadcrumbItem::page("current", page.clone())),
            );
        let basic = Breadcrumb::new("basic").label("Page location").child(
            BreadcrumbList::new()
                .child(self.link("home", "Home", cx))
                .child(BreadcrumbSeparator::new())
                .child(BreadcrumbItem::page("library", page.clone())),
        );
        let bible = Breadcrumb::new("bible").label("Reading location").child(
            BreadcrumbList::new()
                .child(self.link("bible-home", "Home", cx))
                .child(BreadcrumbSeparator::new())
                .child(BreadcrumbItem::page("bible-book", "Genesis"))
                .child(BreadcrumbSeparator::new())
                .child(BreadcrumbItem::page("bible-chapter", "1"))
                .child(BreadcrumbSeparator::new())
                .child(BreadcrumbItem::page("bible-verse", "1")),
        );
        let chapter_picker = Breadcrumb::new("chapter-picker")
            .label("Chapter picker")
            .child(
                BreadcrumbList::new()
                    .child(self.link("picker-home", "Home", cx))
                    .child(BreadcrumbSeparator::new())
                    .child(Self::menu("book", "Genesis", self.chapters.clone()))
                    .child(BreadcrumbSeparator::new())
                    .child(Self::toc("chapter", "1", self.chapters.clone()))
                    .child(BreadcrumbSeparator::new())
                    .child(BreadcrumbItem::page("verse", "1")),
            );
        let separators = div().flex().flex_col().gap_4().children(
            ["Chevron", "Slash", "Custom symbol", "Menu button"]
                .into_iter()
                .map(|name| {
                    let separator = match name {
                        "Slash" => {
                            BreadcrumbSeparator::new().symbol(BreadcrumbSeparatorSymbol::Slash)
                        }
                        "Custom symbol" => BreadcrumbSeparator::new().content("·"),
                        "Menu button" => {
                            let picker = self.sections.clone();
                            BreadcrumbSeparator::menu(
                                "location-menu",
                                "Choose a location",
                                move |trigger| picker_popup("separator-menu", trigger, picker),
                            )
                        }
                        _ => BreadcrumbSeparator::new(),
                    };
                    div()
                        .flex()
                        .flex_col()
                        .gap_1()
                        .child(
                            div()
                                .text_xs()
                                .text_color(cx.theme().colors.fg_muted)
                                .child(name),
                        )
                        .child(
                            Breadcrumb::new(name)
                                .label(format!("{name} example"))
                                .child(
                                    BreadcrumbList::new()
                                        .child(self.link("separator-home", "Home", cx))
                                        .child(separator)
                                        .child(BreadcrumbItem::page(
                                            "separator-page",
                                            page.clone(),
                                        )),
                                ),
                        )
                }),
        );
        let composition = Breadcrumb::new("composition")
            .label("Custom composition")
            .child(
                BreadcrumbList::new()
                    .child(self.link("composition-home", "Home", cx))
                    .child(BreadcrumbSeparator::new())
                    .child(BreadcrumbEllipsis)
                    .child(BreadcrumbSeparator::new())
                    .child(BreadcrumbItem::page("composition-page", page)),
            );
        div().id("ui-showcase").w_full().min_w_0().flex().flex_col().gap_8()
            .child(div().flex().flex_col().gap_2().child(Heading::h1("Breadcrumb"))
                .child(Paragraph::new("Composed navigation with links, a table of contents, and searchable menus.")))
            .child(example("All item variants", "Open Contents or Sections to choose a location.", trail, cx))
            .child(div().flex().flex_col().gap_2()
                .child(div().text_sm().font_medium().child("Current page"))
                .child(div().max_w(gpui::rems(24.0)).child(Input::new(&self.current)))
                .child(div().id("breadcrumb-selection").text_sm().text_color(cx.theme().colors.fg_muted).child(self.selected.clone())))
            .child(example("Link and current page", "The current location is noninteractive.", basic, cx))
            .child(example("Bible location", "Book, chapter, and verse are noninteractive current-location items.", bible, cx))
            .child(example("Separator symbols and menu", "Chevron, slash, custom content, and an icon-sized menu trigger.", separators, cx))
            .child(example("Composition", "Root, list, link, separator, page, and a decorative ellipsis.", composition, cx))
            .child(example("Chapter picker composition", "Choose a chapter from Genesis or open its contents.", chapter_picker, cx))
    }
}

struct Picker {
    title: &'static str,
    items: &'static [&'static str],
    query: Entity<TextInput>,
    highlighted: Option<&'static str>,
    selected: Option<&'static str>,
    owner: WeakEntity<Ui>,
    _subscription: Subscription,
}

enum PickerStep {
    Previous,
    Next,
    First,
    Last,
}

impl Picker {
    fn new(
        title: &'static str,
        items: &'static [&'static str],
        owner: WeakEntity<Ui>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let query = cx.new(|cx| {
            TextInput::new(window, cx)
                .label(if title == "Table of Content" {
                    "Search table of contents"
                } else {
                    "Search commands"
                })
                .placeholder(if title == "Table of Content" {
                    "Search…"
                } else {
                    "Search commands…"
                })
        });
        let subscription = cx.subscribe(&query, |this, _, event, cx| {
            if *event == InputEvent::Changed {
                this.highlighted = None;
                cx.notify();
            }
        });
        Self {
            title,
            items,
            query,
            highlighted: None,
            selected: None,
            owner,
            _subscription: subscription,
        }
    }

    fn filtered(&self, cx: &App) -> Vec<&'static str> {
        let query = self.query.read(cx).text().trim().to_lowercase();
        self.items
            .iter()
            .copied()
            .filter(|item| item.to_lowercase().contains(&query))
            .collect()
    }

    fn choose(&mut self, label: &'static str, window: &mut Window, cx: &mut Context<Self>) {
        self.selected = Some(label);
        self.owner
            .update(cx, |owner, cx| owner.select(label, cx))
            .ok();
        cx.notify();
        window.dispatch_action(Box::new(gpui_kit::base::actions::Cancel), cx);
    }

    fn highlight(&mut self, movement: PickerStep, cx: &mut Context<Self>) {
        let items = self.filtered(cx);
        if items.is_empty() {
            return;
        }
        let ix = self
            .highlighted
            .and_then(|label| items.iter().position(|item| *item == label))
            .unwrap_or(0);
        let next = match movement {
            PickerStep::Previous => (ix + items.len() - 1) % items.len(),
            PickerStep::Next => (ix + 1) % items.len(),
            PickerStep::First => 0,
            PickerStep::Last => items.len() - 1,
        };
        self.highlighted = Some(items[next]);
        cx.notify();
    }
}

impl Render for Picker {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let items = self.filtered(cx);
        let active = self
            .highlighted
            .filter(|label| items.contains(label))
            .or(items.first().copied());
        let is_toc = self.title == "Table of Content";
        let width = gpui::rems(if is_toc { 16.0 } else { 28.0 })
            .to_pixels(window.rem_size())
            .min(window.viewport_size().width - window.rem_size() * 4.0);
        div()
            .id("breadcrumb-picker")
            .w(width)
            .min_w_0()
            .flex()
            .flex_col()
            .gap_3()
            .key_context("BreadcrumbPicker")
            .on_action(
                cx.listener(|this, _: &Previous, _, cx| this.highlight(PickerStep::Previous, cx)),
            )
            .on_action(cx.listener(|this, _: &Next, _, cx| this.highlight(PickerStep::Next, cx)))
            .on_action(cx.listener(|this, _: &First, _, cx| this.highlight(PickerStep::First, cx)))
            .on_action(cx.listener(|this, _: &Last, _, cx| this.highlight(PickerStep::Last, cx)))
            .on_action(cx.listener(|this, _: &Choose, window, cx| {
                let items = this.filtered(cx);
                if let Some(label) = this
                    .highlighted
                    .filter(|label| items.contains(label))
                    .or(items.first().copied())
                {
                    this.choose(label, window, cx);
                }
            }))
            .on_action(cx.listener(|this, _: &FocusSearch, window, cx| {
                this.query.focus_handle(cx).focus(window, cx)
            }))
            .when(is_toc, |picker| {
                picker.child(div().text_lg().font_bold().child(self.title))
            })
            .child(
                Input::new(&self.query)
                    .prefix(
                        Icon::new(IconName::Search)
                            .size(IconSize::Sm)
                            .color(cx.theme().colors.fg_muted),
                    )
                    .when(is_toc, |input| {
                        input.suffix(
                            div()
                                .text_xs()
                                .px_1()
                                .rounded(cx.theme().radius(Radius::Sm))
                                .border_1()
                                .border_color(cx.theme().colors.border)
                                .text_color(cx.theme().colors.fg_subtle)
                                .child(crate::app::keys::label("secondary-k")),
                        )
                    }),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .children(items.iter().map(|&label| {
                        div()
                            .id(label)
                            .key_context("BreadcrumbPickerRow")
                            .on_hover(cx.listener(move |this, hovered, _, cx| {
                                if *hovered && this.highlighted != Some(label) {
                                    this.highlighted = Some(label);
                                    cx.notify();
                                }
                            }))
                            .on_action(cx.listener(move |this, _: &Choose, window, cx| {
                                this.choose(label, window, cx)
                            }))
                            .child(
                                Button::new(label)
                                    .accessibility_label(label)
                                    .h_8()
                                    .w_full()
                                    .px_3()
                                    .justify_start()
                                    .rounded(cx.theme().radius(Radius::Sm))
                                    .text_sm()
                                    .text_color(cx.theme().colors.fg)
                                    .bg(if active == Some(label) {
                                        cx.theme().colors.hover
                                    } else {
                                        gpui::transparent_black()
                                    })
                                    .child(if self.selected == Some(label) {
                                        format!("✓ {label}")
                                    } else {
                                        label.to_string()
                                    })
                                    .on_click(cx.listener(move |this, _, window, cx| {
                                        this.choose(label, window, cx)
                                    })),
                            )
                    }))
                    .when(items.is_empty(), |list| {
                        list.child(
                            div()
                                .p_3()
                                .text_sm()
                                .text_color(cx.theme().colors.fg_muted)
                                .child("No matching locations"),
                        )
                    }),
            )
    }
}

#[cfg(test)]
#[path = "ui.test.rs"]
mod tests;
