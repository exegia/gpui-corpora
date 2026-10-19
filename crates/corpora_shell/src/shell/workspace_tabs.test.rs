use super::WorkspaceTabs;

#[test]
fn navigation_without_added_tabs_keeps_the_strip_empty() {
    let mut tabs = WorkspaceTabs::default();
    tabs.navigated("/explore");
    tabs.navigated("/projects");
    assert!(tabs.entries().is_empty());
}

#[test]
fn plus_always_adds_a_separate_empty_tab() {
    let mut tabs = WorkspaceTabs::default();
    let first = tabs.new_empty();
    let second = tabs.new_empty();
    assert_ne!(first, second);
    assert_eq!(tabs.entries().len(), 2);
    assert_eq!(tabs.active_id(), Some(second.as_str()));
}

#[test]
fn navigation_leaves_added_tabs_and_their_empty_pages_unchanged() {
    let mut tabs = WorkspaceTabs::default();
    let first = tabs.new_empty();
    tabs.navigated("/projects");
    let second = tabs.new_empty();
    tabs.navigated("/explore");
    tabs.navigated("/explore/detail/text/one");
    assert_eq!(tabs.entries().len(), 2);
    assert_eq!(tabs.entries()[0].number, 1);
    assert_eq!(tabs.active_id(), None);
    assert_eq!(tabs.select(&first), Some(first.clone()));
    assert_eq!(tabs.select(&second), Some(second.clone()));
}

#[test]
fn closing_the_active_tab_selects_its_neighbor_then_home() {
    let mut tabs = WorkspaceTabs::default();
    let first = tabs.new_empty();
    let second = tabs.new_empty();
    assert_eq!(tabs.close(&second), Some(first.clone()));
    assert_eq!(tabs.active_id(), Some(first.as_str()));
    assert_eq!(tabs.close(&first).as_deref(), Some("/"));
    assert!(tabs.entries().is_empty());
}

#[test]
fn top_level_navigation_deselects_tabs_without_creating_entries() {
    let mut tabs = WorkspaceTabs::default();
    let tab = tabs.new_empty();
    for route in [
        "/",
        "/explore",
        "/projects",
        "/settings",
        "/profile/information",
    ] {
        tabs.select(&tab);
        tabs.navigated(route);
        assert_eq!(tabs.active_id(), None);
        assert_eq!(tabs.entries().len(), 1);
        assert_eq!(tabs.entries()[0].path, tab);
    }
}

#[test]
fn closing_a_background_tab_keeps_the_active_tab() {
    let mut tabs = WorkspaceTabs::default();
    let first = tabs.new_empty();
    let second = tabs.new_empty();
    assert_eq!(tabs.close(&first), None);
    assert_eq!(tabs.active_id(), Some(second.as_str()));
}

#[test]
fn restored_empty_tab_ids_are_not_reused() {
    let mut tabs = WorkspaceTabs::default();
    tabs.restore_empty("/tabs/new/7");
    assert_eq!(tabs.new_empty(), "/tabs/new/8");
}

#[test]
fn restoring_a_normal_route_does_not_create_a_tab() {
    let mut tabs = WorkspaceTabs::default();
    tabs.restore_empty("/projects");
    assert!(tabs.entries().is_empty());
}
