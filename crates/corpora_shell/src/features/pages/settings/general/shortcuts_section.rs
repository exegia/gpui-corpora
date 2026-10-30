#[allow(unused_imports)]
use super::*;

pub fn shortcuts_section(cx: &App) -> impl IntoElement + use<> {
    let enabled = crate::app_state::config(cx).global_shortcut_enabled;
    SettingsSection::new(crate::i18n::localize("settings_shortcuts")).row(
        SettingsRow::new(crate::i18n::localize("settings_global_shortcut")).control(
            Switch::new("global-shortcut-enabled", enabled)
                .name(crate::i18n::localize("settings_global_shortcut"))
                .on_change(|on, _, cx| {
                    crate::app_state::update_config(cx, |config| {
                        config.global_shortcut_enabled = on;
                    });
                    crate::shortcuts::apply_enabled(on, cx);
                }),
        ),
    )
}
