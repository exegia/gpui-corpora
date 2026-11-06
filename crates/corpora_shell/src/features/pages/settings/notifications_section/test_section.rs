use super::*;

pub fn test_section() -> impl IntoElement {
    SettingsSection::new(crate::i18n::localize("settings_section_notifications"))
        .row(super::super::button_row([Button::new(
            "test-native-notification",
            crate::i18n::localize("settings_test_native_notification"),
        )
        .primary()
        .on_click(|_, _, cx| {
            notifications::send(
                NotificationRequest::test_notification(
                    crate::i18n::localize("settings_test_native_notification"),
                    crate::i18n::localize("settings_hello_notification"),
                ),
                cx,
            );
        })]))
        .row(super::super::button_row([
            test_button(
                "test-action-notification",
                "settings_test_action_notification",
                |title| {
                    NotificationRequest::action_buttons(
                        title,
                        crate::i18n::localize("settings_action_notification_body"),
                    )
                },
            ),
            test_button(
                "test-reply-notification",
                "settings_test_reply_notification",
                |title| {
                    NotificationRequest::reply(
                        title,
                        crate::i18n::localize("settings_reply_notification_body"),
                    )
                },
            ),
            test_button(
                "test-background-worthy-notification",
                "settings_test_background_notification",
                |title| {
                    NotificationRequest::background_worthy(
                        title,
                        crate::i18n::localize("settings_background_notification_body"),
                    )
                },
            ),
        ]))
}
