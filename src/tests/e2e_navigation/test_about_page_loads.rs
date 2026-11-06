#[allow(unused_imports)]
use super::*;

/// AboutPage is a zero-argument struct, instantiable without GPUI context.
#[test]
pub(crate) fn test_about_page_loads() {
    let _page = gpui_starter::views::AboutPage::new();
}
