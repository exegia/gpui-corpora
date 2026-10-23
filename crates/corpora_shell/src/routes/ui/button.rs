use corpora_ui::{
    atoms::button::{Button, ButtonSize, ButtonVariant},
    ely::{
        primitives::IconName,
        theme::ActiveTheme,
        typography::{Heading, Paragraph},
    },
};
use gpui::{App, Context, IntoElement, Render, Window, div, prelude::*};

/// Button playground state retained by the workspace tab.
pub struct ButtonShowcase {
    variant: ButtonVariant,
    size: ButtonSize,
    loading: bool,
    disabled: bool,
    clicks: usize,
}

impl ButtonShowcase {
    pub fn new() -> Self {
        Self {
            variant: ButtonVariant::Default,
            size: ButtonSize::Default,
            loading: false,
            disabled: false,
            clicks: 0,
        }
    }
}

fn control_group(
    label: &'static str,
    children: impl IntoIterator<Item = Button>,
    cx: &App,
) -> gpui::Div {
    div()
        .flex()
        .flex_col()
        .gap_2()
        .child(
            div()
                .text_xs()
                .text_color(cx.theme().colors.fg_muted)
                .child(label),
        )
        .child(
            div()
                .flex()
                .flex_wrap()
                .items_center()
                .gap_1()
                .children(children),
        )
}

fn choice(id: (&'static str, &'static str), name: &'static str, selected: bool) -> Button {
    let label = format!("{} {name}", id.0);
    let id = (gpui::ElementId::from(id.0), id.1);
    Button::new(id, label)
        .child(name)
        .size(ButtonSize::Sm)
        .variant(if selected {
            ButtonVariant::Secondary
        } else {
            ButtonVariant::Ghost
        })
}

impl Render for ButtonShowcase {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let preview = Button::new(
            "button-preview",
            if self.size.is_icon() {
                "Open manuscript"
            } else {
                "Button"
            },
        )
        .variant(self.variant)
        .size(self.size)
        .loading(self.loading)
        .disabled(self.disabled)
        .when(self.size.is_icon(), |button| {
            button.icon(IconName::BookOpen)
        })
        .on_click(cx.listener(|this, _, _, cx| {
            this.clicks += 1;
            cx.notify();
        }));
        let demo = div()
            .flex()
            .items_center()
            .justify_center()
            .p_6()
            .min_h(gpui::rems(7.))
            .child(preview);
        let controls = div()
            .flex()
            .flex_col()
            .gap_4()
            .child(control_group(
                "Variant",
                ButtonVariant::ALL.map(|variant| {
                    choice(
                        ("variant", variant.name()),
                        variant.name(),
                        self.variant.name() == variant.name(),
                    )
                    .on_click(cx.listener(move |this, _, _, cx| {
                        this.variant = variant;
                        cx.notify();
                    }))
                }),
                cx,
            ))
            .child(control_group(
                "Size",
                ButtonSize::ALL.map(|size| {
                    choice(("size", size.name()), size.name(), self.size == size).on_click(
                        cx.listener(move |this, _, _, cx| {
                            this.size = size;
                            cx.notify();
                        }),
                    )
                }),
                cx,
            ))
            .child(
                div()
                    .flex()
                    .flex_wrap()
                    .gap_6()
                    .child(control_group(
                        "Loading",
                        [false, true].map(|value| {
                            choice(
                                ("loading", if value { "true" } else { "false" }),
                                if value { "true" } else { "false" },
                                self.loading == value,
                            )
                            .on_click(cx.listener(
                                move |this, _, _, cx| {
                                    this.loading = value;
                                    cx.notify();
                                },
                            ))
                        }),
                        cx,
                    ))
                    .child(control_group(
                        "Disabled",
                        [false, true].map(|value| {
                            choice(
                                ("disabled", if value { "true" } else { "false" }),
                                if value { "true" } else { "false" },
                                self.disabled == value,
                            )
                            .on_click(cx.listener(
                                move |this, _, _, cx| {
                                    this.disabled = value;
                                    cx.notify();
                                },
                            ))
                        }),
                        cx,
                    )),
            )
            .child(
                div()
                    .id("button-click-count")
                    .text_sm()
                    .text_color(cx.theme().colors.fg_muted)
                    .child(format!("Activated {} times", self.clicks)),
            );
        let variants =
            div()
                .flex()
                .flex_wrap()
                .items_center()
                .gap_3()
                .children(ButtonVariant::ALL.map(|variant| {
                    Button::new(
                        (gpui::ElementId::from("example-variant"), variant.name()),
                        variant.name(),
                    )
                    .variant(variant)
                }));
        let sizes = div()
            .flex()
            .flex_wrap()
            .items_center()
            .gap_3()
            .children(ButtonSize::ALL.map(|size| {
                Button::new(
                    (gpui::ElementId::from("example-size"), size.name()),
                    if size.is_icon() {
                        "Open manuscript"
                    } else {
                        size.name()
                    },
                )
                .size(size)
                .variant(ButtonVariant::Outline)
                .when(size.is_icon(), |button| button.icon(IconName::BookOpen))
            }));
        let states = div()
            .flex()
            .flex_wrap()
            .items_center()
            .gap_3()
            .child(Button::new("example-loading", "Saving").loading(true))
            .child(
                Button::new("example-icon-loading", "Saving manuscript")
                    .size(ButtonSize::Icon)
                    .loading(true)
                    .icon(IconName::BookOpen),
            )
            .child(Button::new("example-disabled", "Unavailable").disabled(true))
            .child(
                Button::new("example-composition", "Open manuscript")
                    .variant(ButtonVariant::Outline)
                    .icon(IconName::BookOpen),
            )
            .child(
                Button::new("example-link", "Button documentation")
                    .variant(ButtonVariant::Link)
                    .href(
                        "https://corpora-ui-git-release-v500-corpora-apps.vercel.app/atoms/button",
                    ),
            );
        div()
            .id("button-showcase")
            .w_full()
            .min_w_0()
            .flex()
            .flex_col()
            .gap_8()
            .child(super::component_navigation(true))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(Heading::h1("Button"))
                    .child(Paragraph::new(
                        "Explore the Button atom’s variants, sizes, loading, and disabled states.",
                    )),
            )
            .child(super::example(
                "Playground",
                "Change the controls, then activate Button with a click, Enter, or Space.",
                div().flex().flex_col().gap_6().child(demo).child(controls),
                cx,
            ))
            .child(super::example(
                "Variants",
                "Primary, secondary, outline, destructive, ghost, and link treatments.",
                variants,
                cx,
            ))
            .child(super::example(
                "Sizes",
                "Five text sizes and five square icon sizes.",
                sizes,
                cx,
            ))
            .child(super::example(
                "States and composition",
                "Text stays visible while loading. Icon sizes overlay the spinner.",
                states,
                cx,
            ))
    }
}

#[cfg(test)]
#[path = "button.test.rs"]
mod tests;
