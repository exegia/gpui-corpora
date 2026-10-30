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
fn restored_route_page_shows_without_a_tab() {
    let mut tabs = WorkspaceTabs::default();
    assert!(!tabs.show("/projects"));
    tabs.open("/projects", State::default());
    assert!(tabs.entries().is_empty());
    assert_eq!(tabs.active().unwrap().path, "/projects");
}

#[test]
fn route_pages_share_one_view_and_follow_its_router() {
    let mut tabs = WorkspaceTabs::default();
    tabs.open("/", "route state");
    assert!(tabs.show("/explore"));
    tabs.navigated("/explore/detail/text/one");
    assert!(tabs.entries().is_empty());
    let route = tabs.active().unwrap();
    assert_eq!(route.path, "/explore/detail/text/one");
    assert_eq!(route.state, "route state");
}

#[test]
fn leaving_a_tab_for_a_route_page_keeps_the_tab_and_its_path() {
    let mut tabs = WorkspaceTabs::default();
    tabs.open("/", ());
    let tab = tabs.new_empty(());
    assert!(tabs.show("/projects"));
    tabs.navigated("/projects");
    assert_ne!(tabs.active_id(), Some(tab.as_str()));
    assert!(tabs.show(&tab));
    tabs.navigated("/projects");
    assert_eq!(tabs.active().unwrap().path, tab);
    assert_eq!(tabs.entries().len(), 1);
}

#[test]
fn tabs_keep_independent_state() {
    let mut tabs = WorkspaceTabs::default();
    let first = tabs.new_empty(State::default());
    let state = &mut tabs.active_mut().unwrap().state;
    state.sidebar = false;
    state.inspector_width = 28;
    state.selection = "first project".into();
    let second = tabs.new_empty(State::default());
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
fn closing_an_active_tab_shows_its_neighbor() {
    let mut tabs = WorkspaceTabs::default();
    let first = tabs.new_empty("first state");
    let second = tabs.new_empty("second state");
    assert_eq!(tabs.close(&second), Some(first.clone()));
    assert_eq!(tabs.active_id(), Some(first.as_str()));
    assert_eq!(tabs.active().unwrap().state, "first state");
}

#[test]
fn closing_the_last_active_tab_returns_to_the_route_page() {
    let mut tabs = WorkspaceTabs::default();
    tabs.open("/explore", "route state");
    let tab = tabs.new_empty("tab state");
    assert_eq!(tabs.close(&tab).as_deref(), Some("/explore"));
    assert_eq!(tabs.active().unwrap().state, "route state");
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
fn restored_tab_keeps_its_number_and_later_tabs_count_past_it() {
    let mut tabs = WorkspaceTabs::default();
    assert!(!tabs.show("/tabs/new/7"));
    tabs.open("/tabs/new/7", ());
    assert_eq!(tabs.active_id(), Some("/tabs/new/7"));
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
fn closing_the_only_view_drops_its_workspace_and_leaves_no_replacement() {
    let mut tabs = WorkspaceTabs::default();
    let state = Rc::new(());
    let id = tabs.new_empty(state.clone());
    assert_eq!(tabs.close(&id), None);
    assert!(tabs.entries().is_empty());
    assert!(tabs.active_id().is_none());
    assert_eq!(Rc::strong_count(&state), 1);
    assert_eq!(tabs.close(&id), None);
    assert!(tabs.entries().is_empty());
}

#[test]
#[cfg(debug_assertions)]
fn pinned_showcase_keeps_its_subroutes_and_cannot_close() {
    let mut tabs = WorkspaceTabs::default();
    tabs.open("/explore", "route");
    tabs.pin("/ui", "showcase");
    assert_eq!(tabs.active().unwrap().path, "/explore");
    assert!(tabs.show("/ui/button"));
    assert_eq!(tabs.active_id(), Some("/ui"));
    assert_eq!(tabs.close("/ui"), None);
    assert_eq!(tabs.entries().len(), 1);
    assert_eq!(tabs.active().unwrap().path, "/ui/button");
    assert!(!tabs.active().unwrap().closable);
    let tab = tabs.new_empty("new tab");
    assert_eq!(tab, "/tabs/new/1");
    assert_eq!(tabs.close(&tab).as_deref(), Some("/ui/button"));
    assert_eq!(tabs.active().unwrap().state, "showcase");
    assert!(tabs.show("/projects"));
    assert_eq!(tabs.active().unwrap().state, "route");
    assert!(tabs.show("/ui"));
    assert_eq!(tabs.active().unwrap().state, "showcase");
}
