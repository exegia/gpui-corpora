#[allow(unused_imports)]
use super::*;

pub fn section(cx: &App) -> impl IntoElement + use<> {
    let mode = match app::theme::current_mode(cx) {
        ThemeMode::Light => "light",
        ThemeMode::Dark => "dark",
    };
    SettingsSection::new(crate::i18n::localize("settings_section_general"))
        .row(
            SettingsRow::new(crate::i18n::localize("settings_appearance")).control(
                SegmentedControl::new("settings-theme", mode)
                    .segment(
                        "light",
                        crate::i18n::localize("menu_appearance_light"),
                        Some(IconName::Sun),
                    )
                    .segment(
                        "dark",
                        crate::i18n::localize("menu_appearance_dark"),
                        Some(IconName::Moon),
                    )
                    .on_change(|value, _, cx| {
                        let mode = if value.as_ref() == "dark" {
                            ThemeMode::Dark
                        } else {
                            ThemeMode::Light
                        };
                        app::set_theme_mode(mode, cx);
                    }),
            ),
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
