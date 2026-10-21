use corpora_ui::ely::theme::{ControlSize, TextSize};

use super::*;

pub(super) fn tabs(
    workspace: WeakEntity<Workspace>,
    window: &mut Window,
    cx: &mut App,
) -> impl IntoElement + use<> {
    let (entries, active) = workspace
        .upgrade()
        .map(|workspace| {
            let state = workspace.read(cx);
            (
                state.tabs.entries().to_vec(),
                state.tabs.active_id().map(str::to_string),
            )
        })
        .unwrap_or_default();
    let close_side = cx.theme().control_height(ControlSize::Sm) * 0.75;
    let close_top = (cx.theme().control_height(ControlSize::Md) - close_side) * 0.5;
    let tab_min = cx.theme().tab_width().0;
    let text_size = cx.theme().text_size(TextSize::Sm);
    div()
        .id("workspace-tabs")
        .flex()
        .items_center()
        .gap_1()
        .min_w_0()
        .children(entries.into_iter().map(|tab| {
            let selected = active.as_deref() == Some(tab.id.as_str());
            let group = gpui::SharedString::from(format!("workspace-tab-{}", tab.id));
            let select_owner = workspace.clone();
            let select_id = tab.id.clone();
            let close_owner = workspace.clone();
            let close_id = tab.id.clone();
            let label = format!("New tab {}", tab.number);
            let button = Button::new(format!("select-tab-{}", tab.id), label.clone())
                .icon(IconName::File)
                .variant(if selected {
                    ButtonVariant::Secondary
                } else {
                    ButtonVariant::Ghost
                })
                .on_click(move |_, _, cx| {
                    select_owner
                        .update(cx, |state, cx| {
                            if let Some(path) = state.tabs.select(&select_id) {
                                state.navigate(path, cx);
                            }
                        })
                        .ok();
                });
            let button = style_chrome(
                button,
                StyleRefinement::default()
                    .pr_6()
                    .min_w(tab_min)
                    .text_size(text_size),
                window,
                cx,
            );
            let close = IconButton::new(format!("close-tab-{}", tab.id), IconName::X)
                .size(ControlSize::Sm)
                .tooltip(format!("Close {label}"))
                .on_click(move |_, _, cx| {
                    cx.stop_propagation();
                    close_owner
                        .update(cx, |state, cx| {
                            if let Some(path) = state.tabs.close(&close_id) {
                                state.navigate(path, cx);
                            }
                            cx.notify();
                        })
                        .ok();
                });
            let close = style_chrome(
                close,
                StyleRefinement::default().size(close_side),
                window,
                cx,
            );
            div()
                .id(format!("workspace-tab-{}", tab.id))
                .relative()
                .group(group.clone())
                .child(button)
                .child(
                    div()
                        .absolute()
                        .right_1()
                        .top(close_top)
                        .when(!selected, |slot| {
                            slot.invisible().group_hover(group, |style| style.visible())
                        })
                        .child(close),
                )
        }))
}
