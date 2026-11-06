#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn checklist_bridge_claims_anchor_in_code() {
    let palette =
        std::fs::read_to_string("src/features/command_palette.rs").expect("read command palette");
    assert!(
        palette.contains("launcher-status"),
        "palette live region must exist as documented"
    );

    let shell =
        std::fs::read_to_string("src/shell/root/app_root/render.rs").expect("read shell render");
    assert!(
        shell.contains("Role::Navigation"),
        "navigation landmark must exist as documented"
    );
    assert!(
        shell.contains("sidebar_main_navigation"),
        "the landmark's name must carry the localization key"
    );

    let virtual_list =
        std::fs::read_to_string("src/ui/widgets/virtual_list.rs").expect("read virtual list");
    assert!(
        virtual_list.contains("Role::ListItem"),
        "virtual list rows must be list items as documented"
    );
}
