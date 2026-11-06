#[allow(unused_imports)]
use super::*;

/// `secondary` must parse to control outside macOS so Windows builds fire on
/// Ctrl, not the OS-intercepted Windows key.
#[cfg(target_os = "macos")]
#[test]
pub(crate) fn secondary_maps_to_platform_modifier() {
    let stroke = Keystroke::parse(TOGGLE_SEARCH).unwrap();
    assert!(stroke.modifiers.platform);
    assert!(!stroke.modifiers.control);
    assert_eq!(stroke.key.as_str(), "k");
    assert_eq!(label(TOGGLE_SEARCH), "⌘K");
}
