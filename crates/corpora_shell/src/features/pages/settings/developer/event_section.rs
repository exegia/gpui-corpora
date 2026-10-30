#[allow(unused_imports)]
use super::*;

pub(crate) fn event_section(event_log: &[(EventId, String)]) -> impl IntoElement + use<> {
    let mut section = SettingsSection::new(crate::i18n::localize("settings_event_emitter"))
        .description(crate::i18n::localize("settings_event_emitter_desc"))
        .row(super::super::button_row([
            outline("emit-test-noop", "settings_emit_test_noop").on_click(|_, _, cx| {
                crate::events::emit(
                    crate::events::AppEventKind::Test {
                        message: "hello from settings".into(),
                    },
                    cx,
                );
            }),
            outline("emit-navigate-home", "settings_emit_navigate_home").on_click(|_, _, cx| {
                crate::events::emit(
                    crate::events::AppEventKind::Navigate(crate::navigation::HOME.to_string()),
                    cx,
                );
            }),
            outline(
                "emit-navigate-notifications",
                "settings_emit_navigate_notifications",
            )
            .on_click(|_, _, cx| {
                crate::events::emit(
                    crate::events::AppEventKind::Navigate(
                        crate::navigation::NOTIFICATIONS.to_string(),
                    ),
                    cx,
                );
            }),
        ]));
    if event_log.is_empty() {
        section = section.row(Caption::new(crate::i18n::localize("settings_no_events")));
    }
    // Newest first, keyed by event id: the log drains from the front.
    for (id, entry) in event_log.iter().rev() {
        section = section.row(
            div()
                .id(gpui::SharedString::from(format!("settings-event-{id}")))
                .py_1()
                .child(Caption::new(entry.clone())),
        );
    }
    section
}
