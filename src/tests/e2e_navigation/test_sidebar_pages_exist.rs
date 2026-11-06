#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn test_sidebar_pages_exist() {
    let all = Page::all();
    // The app must expose exactly ten pages (nine feature pages + Gallery).
    assert_eq!(
        all.len(),
        10,
        "expected 10 sidebar pages, got {}",
        all.len()
    );

    // Every page variant that AppRoot::active_page_view matches on must be
    // present in the canonical list.
    let expected = [
        Page::Home,
        Page::Form,
        Page::Settings,
        Page::Notifications,
        Page::Diagnostics,
        Page::QueryPlayground,
        Page::QueryDevToolsV2,
        Page::ErrorPlayground,
        Page::About,
    ];

    for page in &expected {
        assert!(
            all.contains(page),
            "Page::{page:?} missing from Page::all()"
        );
    }

    // Each page must return a non-empty title.
    for page in all {
        assert!(
            !page.title().is_empty(),
            "Page::{page:?} has an empty title"
        );
    }
}
