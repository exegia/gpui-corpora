#[allow(unused_imports)]
use super::*;

#[cfg(not(target_os = "macos"))]
#[test]
pub(crate) fn secondary_maps_to_control() {
    let stroke = Keystroke::parse(TOGGLE_SEARCH).unwrap();
    assert!(stroke.modifiers.control);
    assert!(!stroke.modifiers.platform);
    assert_eq!(stroke.key.as_str(), "k");
    assert_eq!(label(TOGGLE_SEARCH), "Ctrl+K");
}
