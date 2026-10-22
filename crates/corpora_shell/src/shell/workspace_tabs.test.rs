use std::rc::Rc;

use super::WorkspaceTabs;

#[derive(Debug, PartialEq)]
struct State {
    sidebar: bool,
    inspector_width: u16,
    selection: String,
}

impl Default for State {
    fn default() -> Self {
        Self {
            sidebar: true,
            inspector_width: 20,
            selection: String::new(),
        }
    }
}

#[test]
fn restored_route_has_a_tab_owner() {
    let mut tabs = WorkspaceTabs::default();
    tabs.restore("/projects", State::default());
    assert_eq!(tabs.entries().len(), 1);
    assert_eq!(tabs.active().unwrap().path, "/projects");
}

#[test]
fn navigation_changes_only_the_current_tab() {
    let mut tabs = WorkspaceTabs::default();
    let first = tabs.new_empty(());
    tabs.navigated("/projects");
    let second = tabs.new_empty(());
    tabs.navigated("/explore/detail/text/one");
    assert_eq!(tabs.select(&first).as_deref(), Some("/projects"));
    assert_eq!(
        tabs.select(&second).as_deref(),
        Some("/explore/detail/text/one")
    );
}

#[test]
fn tabs_with_the_same_route_keep_independent_state() {
    let mut tabs = WorkspaceTabs::default();
    let first = tabs.new_empty(State::default());
    tabs.navigated("/projects");
    let state = &mut tabs.active_mut().unwrap().state;
    state.sidebar = false;
    state.inspector_width = 28;
    state.selection = "first project".into();
    let second = tabs.new_empty(State::default());
    tabs.navigated("/projects");
    assert_eq!(tabs.active().unwrap().state, State::default());
    tabs.select(&first);
    assert_eq!(
        tabs.active().unwrap().state,
        State {
            sidebar: false,
            inspector_width: 28,
            selection: "first project".into(),
        }
    );
    assert_eq!(tabs.active_id(), Some(first.as_str()));
    tabs.select(&second);
    assert_eq!(tabs.active().unwrap().state, State::default());
}

#[test]
fn closing_an_active_tab_restores_its_neighbors_route_and_state() {
    let mut tabs = WorkspaceTabs::default();
    let first = tabs.new_empty("first state");
    tabs.navigated("/explore");
    let second = tabs.new_empty("second state");
    tabs.navigated("/projects");
    assert_eq!(tabs.close(&second).as_deref(), Some("/explore"));
    assert_eq!(tabs.active_id(), Some(first.as_str()));
    assert_eq!(tabs.active().unwrap().state, "first state");
}

#[test]
fn closing_a_background_tab_releases_its_state_without_switching_tabs() {
    let mut tabs = WorkspaceTabs::default();
    let state = Rc::new(());
    let first = tabs.new_empty(state.clone());
    let second = tabs.new_empty(Rc::new(()));
    assert_eq!(tabs.close(&first), None);
    assert_eq!(Rc::strong_count(&state), 1);
    assert_eq!(tabs.active_id(), Some(second.as_str()));
}

#[test]
fn closing_the_last_tab_does_not_reuse_its_identity() {
    let mut tabs = WorkspaceTabs::default();
    let first = tabs.new_empty(());
    tabs.close(&first);
    assert!(tabs.active().is_none());
    assert_ne!(tabs.new_empty(()), first);
}

#[test]
fn restored_empty_tab_ids_are_not_reused() {
    let mut tabs = WorkspaceTabs::default();
    tabs.restore("/tabs/new/7", ());
    assert_eq!(tabs.new_empty(()), "/tabs/new/8");
}

#[test]
fn invalid_selection_preserves_the_current_tab() {
    let mut tabs = WorkspaceTabs::default();
    let first = tabs.new_empty(());
    assert_eq!(tabs.select("missing"), None);
    assert_eq!(tabs.close("missing"), None);
    assert_eq!(tabs.active_id(), Some(first.as_str()));
}

#[test]
fn closing_the_only_tab_drops_its_workspace_and_leaves_no_replacement() {
    let mut tabs = WorkspaceTabs::default();
    let state = Rc::new(());
    let id = tabs.new_empty(state.clone());
    tabs.close(&id);
    assert!(tabs.entries().is_empty());
    assert!(tabs.active_id().is_none());
    assert_eq!(Rc::strong_count(&state), 1);
    assert_eq!(tabs.close(&id), None);
    assert!(tabs.entries().is_empty());
}
