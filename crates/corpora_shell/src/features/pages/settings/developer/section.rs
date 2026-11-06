#[allow(unused_imports)]
use super::*;

pub fn section(event_log: &[(EventId, String)], cx: &App) -> impl IntoElement + use<> {
    let show_frame_time = crate::app_state::config(cx).show_frame_time;
    div()
        .flex()
        .flex_col()
        .gap_8()
        .child(
            SettingsSection::new(crate::i18n::localize("settings_developer")).row(
                SettingsRow::new(crate::i18n::localize("settings_show_frame_time")).control(
                    Switch::new("show-frame-time", show_frame_time)
                        .name(crate::i18n::localize("settings_show_frame_time"))
                        .on_change(|on, _, cx| {
                            crate::app_state::update_config(cx, |config| {
                                config.show_frame_time = on;
                            });
                        }),
                ),
            ),
        )
        .child(desktop_actions_section())
        .child(telemetry_section())
        .child(runtime_boundaries_section())
        .child(event_section(event_log))
}
