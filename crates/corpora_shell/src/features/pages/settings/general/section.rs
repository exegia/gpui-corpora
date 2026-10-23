#[allow(unused_imports)]
use super::*;

use gpui_kit::component::setting::{SettingField, SettingGroup, SettingItem};

pub fn section() -> SettingGroup {
    SettingGroup::new()
        .title(crate::i18n::localize("settings_section_general"))
        .item(
            SettingItem::new(
                crate::i18n::localize("settings_appearance"),
                SettingField::render(|_, _, cx| theme_picker::theme_picker(cx)),
            )
            .layout(gpui::Axis::Vertical),
        )
        .item(SettingItem::new(
            crate::i18n::localize("settings_language"),
            SettingField::dropdown(
                vec![
                    (
                        LOCALE_EN.into(),
                        crate::i18n::localize("settings_language_english").into(),
                    ),
                    (
                        LOCALE_ZH_CN.into(),
                        crate::i18n::localize("settings_language_simplified_chinese").into(),
                    ),
                ],
                |cx: &App| app::current_locale(cx),
                |value: SharedString, cx| app::set_locale(&value, cx),
            ),
        ))
}
