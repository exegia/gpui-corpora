#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn playground_helpers_derive_ids_from_call_site_keys() {
    let helpers = std::fs::read_to_string("src/features/pages/query_playground/ui_helpers.rs")
        .expect("read playground ui helpers");

    for (keyed, text_derived) in [
        ("pg-mini-title-{key}", "pg-mini-title-{}"),
        ("pg-status-{key}", "pg-status-{}"),
        ("pg-chip-{key}", "pg-chip-{}"),
    ] {
        assert!(
            helpers.contains(keyed),
            "{keyed} must stay: ids derive from the per-call-site key"
        );
        assert!(
            !helpers.contains(text_derived),
            "{text_derived} is the collision shape: text repeats across call sites"
        );
    }
}
