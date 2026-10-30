#[allow(unused_imports)]
use super::*;

/// SettingsPage is exported with the expected constructor; full construction
/// needs GPUI globals this test deliberately avoids.
#[test]
pub(crate) fn test_settings_page_loads() {
    // Verify the route parses correctly.
    let route = AppRoute::parse_deep_link("gpui-starter://settings").unwrap();
    assert_eq!(route, AppRoute::Page(Page::Settings));
    assert_eq!(route.page_for_render(), Page::Settings);

    // Compile-time check of the constructor signature only.
    let _type_check: fn() = || {
        fn _assert_settings_page_constructible() {
            // Compile-time proof of the public constructor signature.
            let _: fn(
                &mut gpui_kit::Window,
                &mut gpui_kit::Context<gpui_starter::views::SettingsPage>,
            ) -> gpui_starter::views::SettingsPage = gpui_starter::views::SettingsPage::new;
        }
    };
}
