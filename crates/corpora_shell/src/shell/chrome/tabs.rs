use corpora_ui::ely::theme::{ControlSize, TextSize};
use gpui::px;
use gpui_kit::base::{Transition, motion::Presence};

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
    let new_tab_owner = workspace.clone();
    div()
        .id("workspace-tabs")
        .flex()
        .items_center()
        .gap_1()
        .min_w_0()
        .children(entries.into_iter().map(|tab| {
            let selected = active.as_deref() == Some(tab.id.as_str());
            let group = gpui::SharedString::from(format!("workspace-tab-{}", tab.id));
            let motion = gpui_kit::component::Theme::global(cx).motion_tokens();
            let glass = Presence::new((group.clone(), "glass-selection"), selected)
                .transition(
                    Transition::new(motion.duration_fast).easing(motion.easing_move.clone()),
                )
                .sample(window, cx);
            let select_owner = workspace.clone();
            let select_id = tab.id.clone();
            let close_owner = workspace.clone();
            let close_id = tab.id.clone();
            let label = if tab.closable {
                format!("New tab {}", tab.number)
            } else {
                "UI".to_string()
            };
            let button = Button::new(format!("select-tab-{}", tab.id), label.clone())
                .icon(IconName::AppWindow)
                .variant(ButtonVariant::Ghost)
                .on_click(move |_, _, cx| {
                    select_owner
                        .update(cx, |state, cx| {
                            if let Some(path) = state.tabs.path(&select_id) {
                                state.navigate(path, cx);
                            }
                        })
                        .ok();
                });
            let button = style_chrome(
                button,
                StyleRefinement::default()
                    .pr_6()
                    .bg(gpui::transparent_black())
                    .text_color(cx.theme().colors.fg)
                    .min_w(tab_min)
                    .text_size(text_size),
                window,
                cx,
            );
            let close = IconButton::new(format!("close-tab-{}", tab.id), IconName::X)
                .size(ControlSize::Sm)
                .disabled(!tab.closable)
                .tooltip(format!("Close {label}"))
                .on_click(move |_, _, cx| {
                    cx.stop_propagation();
                    close_owner
                        .update(cx, |state, cx| {
                            state.close_tab(&close_id, cx);
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
                .when(glass.should_render(), |tab| {
                    tab.child(
                        div()
                            .absolute()
                            .inset_0()
                            .opacity(glass.progress)
                            .child(corpora_ui::ui::glass::clear(px(8.))),
                    )
                })
                .child(button)
                .child(
                    div()
                        .absolute()
                        .right_1()
                        .top(close_top)
                        .when(!selected && tab.closable, |slot| {
                            slot.invisible().group_hover(group, |style| style.visible())
                        })
                        .child(close),
                )
        }))
        .child(
            IconButton::new("new-workspace-tab", IconName::Plus)
                .tooltip("New tab")
                .on_click(move |_, window, cx| {
                    new_tab_owner
                        .update(cx, |state, cx| state.new_tab(window, cx))
                        .ok();
                }),
        )
}
