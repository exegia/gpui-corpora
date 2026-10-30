#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn virtual_list_pins_zero_based_positions_and_container_setsize() {
    let virtual_list =
        std::fs::read_to_string("src/ui/widgets/virtual_list.rs").expect("read virtual list");

    assert!(
        virtual_list.contains(".aria_position_in_set(index)"),
        "accesskit stores position_in_set 0-based; AT bridges report stored+1"
    );
    assert!(
        !virtual_list.contains(".aria_position_in_set(index + 1)"),
        "a +1 here reads one high through every AT bridge"
    );
    // 2 = the container call (the only one the bridge reads) + the item-level
    // call; item-only setsize surfaces as none through the ancestor walk.
    assert_eq!(
        virtual_list.matches(".aria_size_of_set(total)").count(),
        2,
        "the bridge walks ancestors only, so the List container must declare setsize"
    );
}
