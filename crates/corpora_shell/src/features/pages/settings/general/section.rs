#[allow(unused_imports)]
use super::*;

pub fn section(cx: &App) -> impl IntoElement + use<> {
    SettingsSection::new(crate::i18n::localize("settings_section_general"))
        .row(
            SettingsRow::new(crate::i18n::localize("settings_appearance"))
                .control(theme_picker::theme_picker(cx)),
        )
        .row(
            SettingsRow::new(crate::i18n::localize("settings_language")).control(
                SegmentedControl::new("settings-language", app::current_locale(cx))
                    .segment(
                        LOCALE_EN,
                        crate::i18n::localize("settings_language_english"),
                        None,
                    )
                    .segment(
                        LOCALE_ZH_CN,
                        crate::i18n::localize("settings_language_simplified_chinese"),
                        None,
                    )
                    .on_change(|value, _, cx| app::set_locale(value, cx)),
            ),
        )
}
