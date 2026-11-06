#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn virtual_list_item_chains_with_selection_state() {
    let _row = virtual_list_item("row-0", "gpx-fetch 200 1h ago", 0, 4).aria_selected(true);
}
