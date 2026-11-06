#[allow(unused_imports)]
use super::*;

pub(crate) fn desktop_actions_section() -> impl IntoElement {
    SettingsSection::new(crate::i18n::localize("settings_desktop_actions"))
        .row(super::super::button_row([
            outline("desktop-copy-diagnostics", "diagnostics_copy").on_click(|_, _, cx| {
                let _ = desktop_actions::copy_diagnostics(cx);
            }),
            outline("desktop-open-logs", "diagnostics_open_logs").on_click(|_, _, cx| {
                let _ = desktop_actions::open_logs_folder(cx);
            }),
            outline("desktop-open-config", "diagnostics_open_config").on_click(|_, _, cx| {
                let _ = desktop_actions::open_config_folder(cx);
            }),
        ]))
        .row(super::super::button_row([
            outline("desktop-pick-file", "settings_pick_file").on_click(|_, _, cx| {
                let _ = desktop_actions::pick_file(cx);
            }),
            outline("desktop-pick-folder", "settings_pick_folder").on_click(|_, _, cx| {
                let _ = desktop_actions::pick_folder(cx);
            }),
            outline("desktop-save-file", "settings_save_file").on_click(|_, _, cx| {
                let _ = desktop_actions::save_file(cx);
            }),
        ]))
        .row(super::super::button_row([
            outline("desktop-watch-logs", "settings_watch_logs_dir").on_click(|_, _, cx| {
                let _ = desktop_actions::watch_log_dir(cx);
            }),
            outline("desktop-watch-config", "settings_watch_config_dir").on_click(|_, _, cx| {
                let _ = desktop_actions::watch_config_dir(cx);
            }),
            outline("desktop-unwatch-all", "settings_unwatch_all").on_click(|_, _, cx| {
                let _ = desktop_actions::unwatch_all(cx);
            }),
        ]))
}
