#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn checklist_documents_the_real_bridge() {
    let content = std::fs::read_to_string("docs/accessibility-checklist.md")
        .expect("read accessibility checklist");
    let normalized = content.to_lowercase();

    assert!(normalized.contains("accesskit"));
    assert!(normalized.contains("live region"));
    assert!(normalized.contains("diagnostics"));
    // The bridge is real; the old aspirational wording must stay gone.
    assert!(!normalized.contains("track accesskit integration points"));
}
