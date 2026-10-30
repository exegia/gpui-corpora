#[allow(unused_imports)]
use super::*;

pub(crate) fn language_menu() -> MenuItem {
    let locale = rust_i18n::locale().to_string();
    MenuItem::Submenu(Menu {
        name: crate::i18n::localize("menu_language").into(),
        items: vec![
            MenuItem::action("English", SelectLocale("en".into())).checked(locale == "en"),
            MenuItem::action("简体中文", SelectLocale("zh-CN".into())).checked(locale == "zh-CN"),
        ],
        disabled: false,
    })
}
