use super::{Panes, panes};

#[test]
fn workspace_tabs_split_their_content_and_dock_a_bottom_panel() {
    assert_eq!(
        panes("/tabs/new/3"),
        Panes {
            sidebar: true,
            inspector: true,
            split: true,
            dock: true,
        }
    );
}

#[test]
fn route_pages_keep_one_content_pane_on_every_subpath() {
    let page = Panes {
        sidebar: true,
        inspector: true,
        split: false,
        dock: false,
    };
    assert_eq!(panes("/"), Panes::default());
    assert_eq!(panes("/projects/detail/one"), page);
}
