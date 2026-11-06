//! Owns the outer width so sidebar motion cannot proportionally resize the inspector.

use super::*;
use gpui::{Axis, Context, IntoElement, Pixels, Render, px};
use gpui_kit::base::{HandleEdge, resize_handle};
use std::{cell::Cell, rc::Rc};

struct DragInspector {
    tab: String,
    start_x: Cell<Pixels>,
    width: Pixels,
}

struct DragPreview;
impl Render for DragPreview {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
    }
}

fn limits(window: &Window) -> std::ops::Range<Pixels> {
    let max = rems(32.)
        .to_pixels(window.rem_size())
        .min(window.viewport_size().width * 0.32);
    rems(12.).to_pixels(window.rem_size()).min(max)..max
}

pub(super) fn width(saved: &gpui::Rems, window: &Window) -> Pixels {
    let range = limits(window);
    saved
        .to_pixels(window.rem_size())
        .clamp(range.start, range.end)
}

pub(super) fn layout(
    workspace: WeakEntity<Workspace>,
    tab: &crate::shell::workspace_tabs::WorkspaceTab<crate::shell::tab_workspace::TabWorkspace>,
    center: gpui::AnyElement,
    visible_width: Pixels,
    window: &mut Window,
    cx: &mut App,
) -> gpui::AnyElement {
    let dark = cx.theme().is_dark();
    let saved_width = width(&tab.state.inspector_width, window);
    let resize_owner = workspace.clone();
    let content = div()
        .id("inspector-content")
        .size_full()
        .overflow_y_scroll()
        .track_scroll(&tab.state.inspector_scroll)
        .child(placeholder("Inspector", "No selection", cx));
    div()
        .flex()
        .size_full()
        .min_w_0()
        .on_drag_move::<Rc<DragInspector>>(move |event, window, cx| {
            let drag = event.drag(cx);
            let range = limits(window);
            let value = (drag.width + drag.start_x.get() - event.event.position.x)
                .clamp(range.start, range.end);
            let tab_id = drag.tab.clone();
            workspace
                .update(cx, |owner, cx| {
                    if let Some(tab) = owner.tabs.active_mut().filter(|tab| tab.id == tab_id) {
                        tab.state.inspector_width = rems(value / window.rem_size());
                        cx.notify();
                    }
                })
                .ok();
        })
        .child(div().flex_1().min_w_0().h_full().child(center))
        .when(visible_width > px(0.), |row| {
            row.child(
                div()
                    .id("workspace-inspector")
                    .test_support()
                    .relative()
                    .w(visible_width)
                    .h_full()
                    .flex_none()
                    .overflow_hidden()
                    .child(div().w(saved_width).h_full().pl(rems(0.5)).child(
                        crate::shell::dev_target(
                            "tab-inspector",
                            "Tab inspector",
                            layout::inspector(dark, content),
                        ),
                    ))
                    .child(
                        resize_handle::<DragInspector, DragPreview>(
                            "inspector-resize",
                            Axis::Horizontal,
                        )
                        .inside(HandleEdge::Leading)
                        .with_appearance(splitter())
                        .on_drag(
                            DragInspector {
                                tab: tab.id.clone(),
                                start_x: Cell::new(px(0.)),
                                width: visible_width,
                            },
                            move |drag, _, window, cx| {
                                drag.start_x.set(window.mouse_position().x);
                                let range = limits(window);
                                let value = drag.width.clamp(range.start, range.end);
                                resize_owner
                                    .update(cx, |owner, cx| {
                                        if let Some(tab) =
                                            owner.tabs.active_mut().filter(|tab| tab.id == drag.tab)
                                        {
                                            tab.state.inspector_width =
                                                rems(value / window.rem_size());
                                            tab.state.panels.inspector_visible = true;
                                            cx.notify();
                                        }
                                    })
                                    .ok();
                                cx.stop_propagation();
                                cx.new(|_| DragPreview)
                            },
                        ),
                    ),
            )
        })
        .into_any_element()
}
