use corpora_ui::{
    atoms::tab::{Tab, TabVariant, Tabs},
    ely::{
        primitives::Icon,
        theme::{ControlSize, IconSize},
    },
};

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
    let selected = active
        .as_deref()
        .and_then(|id| entries.iter().position(|tab| tab.id == id));
    let colors = cx.theme().colors.clone();
    let items = entries
        .iter()
        .enumerate()
        .map(|(index, entry)| {
            let label = if entry.closable {
                format!("New tab {}", entry.number)
            } else {
                "UI".to_string()
            };
            let active = selected == Some(index);
            Tab::new(label)
                .icon(
                    Icon::new(IconName::AppWindow)
                        .size(IconSize::Sm)
                        .color(if active {
                            colors.fg
                        } else {
                            colors.fg.opacity(0.6)
                        }),
                )
                .closable(entry.closable)
        })
        .collect::<Vec<_>>();
    let paths = entries
        .iter()
        .map(|entry| entry.path.clone())
        .collect::<Vec<_>>();
    let ids = entries
        .iter()
        .map(|entry| entry.id.clone())
        .collect::<Vec<_>>();
    let select_owner = workspace.clone();
    let close_owner = workspace.clone();
    let new_tab_owner = workspace.clone();

    div()
        .flex()
        .items_center()
        .gap_1()
        .min_w_0()
        .child(
            Tabs::new("workspace-tabs")
                .label("Workspace tabs")
                .variant(TabVariant::Horizontal)
                .selected_index(selected)
                .items(items)
                .on_change(move |index, _, cx| {
                    let Some(path) = paths.get(*index).cloned() else {
                        return;
                    };
                    select_owner
                        .update(cx, |state, cx| state.navigate(path, cx))
                        .ok();
                })
                .on_close(move |index, _, cx| {
                    let Some(id) = ids.get(*index) else {
                        return;
                    };
                    close_owner
                        .update(cx, |state, cx| state.close_tab(id, cx))
                        .ok();
                }),
        )
        .child(round(
            IconButton::new("new-workspace-tab", IconName::Plus)
                .size(ControlSize::Sm)
                .tooltip("New tab")
                .on_click(move |_, window, cx| {
                    new_tab_owner
                        .update(cx, |state, cx| state.new_tab(window, cx))
                        .ok();
                }),
            window,
            cx,
        ))
}
