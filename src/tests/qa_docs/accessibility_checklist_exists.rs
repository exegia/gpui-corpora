#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn accessibility_checklist_exists() {
    let content = std::fs::read_to_string("docs/accessibility-checklist.md")
        .expect("read accessibility checklist");
    assert!(content.contains("Keyboard And Focus"));
    assert!(content.contains("Done Criteria"));
}
