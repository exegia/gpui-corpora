#[allow(unused_imports)]
use super::*;

pub fn storage_section() -> impl IntoElement {
    SettingsSection::new(crate::i18n::localize("settings_storage")).row(super::super::button_row([
        Button::new(
            "storage-health-check",
            crate::i18n::localize("settings_run_health_check"),
        )
        .on_click(|_, _, cx| crate::storage::run_health_check(cx)),
        Button::new(
            "storage-maintenance",
            crate::i18n::localize("settings_run_maintenance"),
        )
        .on_click(|_, _, cx| crate::storage::run_maintenance(cx)),
    ]))
}
