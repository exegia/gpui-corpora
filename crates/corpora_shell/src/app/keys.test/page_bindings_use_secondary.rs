#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn page_bindings_use_secondary() {
    for i in 0..9 {
        let stroke = Keystroke::parse(&page(i)).unwrap();
        assert!(stroke.modifiers.control || stroke.modifiers.platform);
        assert_eq!(stroke.key.as_str(), (i + 1).to_string());
    }
}
