#[allow(unused_imports)]
use super::*;

pub(crate) fn telemetry_section() -> impl IntoElement {
    SettingsSection::new(crate::i18n::localize("settings_telemetry"))
        .description(crate::i18n::localize("settings_telemetry_consent_note"))
        .row(super::super::button_row([
            outline("telemetry-disable", "settings_telemetry_disable").on_click(|_, _, cx| {
                telemetry::set_mode(TelemetryMode::Disabled, false, None, cx);
            }),
            outline("telemetry-local", "settings_telemetry_local_only").on_click(|_, _, cx| {
                telemetry::set_mode(TelemetryMode::LocalOnly, true, None, cx);
            }),
            outline("telemetry-remote", "settings_telemetry_remote").on_click(|_, _, cx| {
                telemetry::set_mode(
                    TelemetryMode::Remote,
                    true,
                    Some("https://telemetry.example.com/v1/events"),
                    cx,
                );
            }),
        ]))
        .row(super::super::button_row([
            outline("telemetry-record-event", "settings_record_test_event")
                .on_click(|_, _, cx| telemetry::record_event("settings_test_event", cx)),
            outline("telemetry-record-error", "settings_record_test_error")
                .on_click(|_, _, cx| telemetry::record_error("settings_test_error", cx)),
            outline("telemetry-flush", "settings_flush_telemetry")
                .on_click(|_, _, cx| telemetry::flush(cx)),
        ]))
}
