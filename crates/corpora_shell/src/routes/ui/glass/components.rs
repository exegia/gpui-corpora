use corpora_ui::{
    atoms::button::{Button, ButtonSize, ButtonVariant},
    ely::theme::{ActiveTheme, Radius},
};
use gpui::{App, ElementId, Hsla, IntoElement, div, prelude::*, px};
use gpui_kit::{base::StyledExt as _, component::collapsible::Collapsible};

use super::model::{Backdrop, ThemeColor};

pub(super) fn choice(
    id: (&'static str, &'static str),
    label: &'static str,
    selected: bool,
) -> Button {
    Button::new((ElementId::from(id.0), id.1), label)
        .child(label)
        .size(ButtonSize::Sm)
        .variant(if selected {
            ButtonVariant::Secondary
        } else {
            ButtonVariant::Ghost
        })
}

pub(super) fn color_choice(
    group: &'static str,
    color: ThemeColor,
    selected: bool,
    disabled: bool,
    cx: &App,
) -> Button {
    Button::new(
        (ElementId::from(group), color.id()),
        format!("{} color {}", group.replace('-', " "), color.id()),
    )
    .size(ButtonSize::Sm)
    .disabled(disabled)
    .variant(if selected {
        ButtonVariant::Secondary
    } else {
        ButtonVariant::Ghost
    })
    .child(
        div()
            .size_3()
            .flex_none()
            .rounded_full()
            .border_1()
            .border_color(cx.theme().colors.border_strong)
            .bg(color.value(cx)),
    )
    .child(color.id())
}

pub(super) fn control_section<T: IntoElement>(
    title: &'static str,
    description: &'static str,
    content: T,
    cx: &App,
) -> impl IntoElement + use<T> {
    div()
        .w_full()
        .flex()
        .flex_col()
        .gap_3()
        .p_4()
        .bg(cx.theme().colors.overlay)
        .rounded(cx.theme().radius(Radius::Xl))
        .border_1()
        .border_color(cx.theme().colors.border)
        .child(
            div()
                .flex()
                .flex_col()
                .gap_1()
                .child(div().text_sm().font_semibold().child(title))
                .child(
                    div()
                        .text_xs()
                        .text_color(cx.theme().colors.fg_muted)
                        .child(description),
                ),
        )
        .child(content)
}

pub(super) fn collapsible_control_section<T: IntoElement, U: IntoElement>(
    trigger: T,
    description: &'static str,
    open: bool,
    content: U,
    cx: &App,
) -> impl IntoElement + use<T, U> {
    Collapsible::new()
        .open(open)
        .w_full()
        .p_4()
        .bg(cx.theme().colors.overlay)
        .rounded(cx.theme().radius(Radius::Xl))
        .border_1()
        .border_color(cx.theme().colors.border)
        .child(trigger)
        .child(
            div()
                .text_xs()
                .text_color(cx.theme().colors.fg_muted)
                .child(description),
        )
        .content(div().w_full().pt_3().child(content))
}

pub(super) fn backdrop(kind: Backdrop, background: Hsla, cx: &App) -> impl IntoElement {
    let colors = cx.theme().colors.clone();
    let base = div().absolute().inset_0().overflow_hidden().bg(background);
    match kind {
        Backdrop::Flat => base,
        Backdrop::Bars => base.child(
            div()
                .absolute()
                .inset_0()
                .flex()
                .children((0..8).map(move |ix| div().flex_1().h_full().bg(colors.chart[ix]))),
        ),
        Backdrop::HorizontalRuler => base.children((0..15).map(move |ix| {
            div()
                .absolute()
                .top(px(ix as f32 * 24.))
                .left_0()
                .right_0()
                .h(px(if ix % 4 == 0 { 3. } else { 1. }))
                .bg(colors.fg.opacity(if ix % 4 == 0 { 0.8 } else { 0.35 }))
        })),
        Backdrop::VerticalRuler => base.children((0..35).map(move |ix| {
            div()
                .absolute()
                .left(px(ix as f32 * 24.))
                .top_0()
                .bottom_0()
                .w(px(if ix % 4 == 0 { 3. } else { 1. }))
                .bg(colors.fg.opacity(if ix % 4 == 0 { 0.8 } else { 0.35 }))
        })),
        Backdrop::Gradient => base.child(div().absolute().inset_0().flex().children((0..32).map(
            move |ix| {
                div()
                    .flex_1()
                    .h_full()
                    .bg(colors.accent.opacity(ix as f32 / 31.))
            },
        ))),
        Backdrop::Text => base.children((0..12).map(move |ix| {
            div()
                .absolute()
                .top(px(8. + ix as f32 * 28.))
                .left(px(12.))
                .right(px(12.))
                .text_2xl()
                .font_semibold()
                .text_color(colors.fg)
                .whitespace_nowrap()
                .overflow_hidden()
                .child("the quick brown fox jumps over the lazy dog and back again")
        })),
        Backdrop::Coded => base.children((0..70).map(move |ix| {
            div()
                .absolute()
                .left(px(ix as f32 * 12.))
                .top_0()
                .bottom_0()
                .w(px(12.))
                .bg(colors.chart[ix % colors.chart.len()]
                    .opacity(0.35 + 0.65 * (ix % 10) as f32 / 9.))
        })),
    }
}
