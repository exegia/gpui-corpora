#[allow(unused_imports)]
use super::*;

pub fn section(snapshot: &NotificationRuntimeSnapshot) -> impl IntoElement + use<> {
    let can_request_permission = snapshot.capabilities.can_request_permission
        && matches!(
            snapshot.permission,
            NotificationPermissionState::NotDetermined
                | NotificationPermissionState::Unknown
                | NotificationPermissionState::Unavailable(_)
        );
    // Linux and Windows have no per-app permission model, so the button routes
    // to the OS notification settings; on macOS only once permission is denied.
    let can_open_settings = cfg!(target_os = "linux")
        || cfg!(target_os = "windows")
        || (cfg!(target_os = "macos")
            && matches!(
                snapshot.permission,
                NotificationPermissionState::Denied | NotificationPermissionState::Unavailable(_)
            ));

    let mut section = SettingsSection::new(crate::i18n::localize("settings_section_notifications"))
        .description(crate::i18n::localize("settings_in_app_notifications_note"))
        .row(
            SettingsRow::new(crate::i18n::localize("settings_native_notifications")).control(
                Switch::new("native-notifications-enabled", snapshot.enabled_by_user)
                    .name(crate::i18n::localize("settings_native_notifications"))
                    .on_change(|on, _, cx| notifications::set_native_notifications_enabled(on, cx)),
            ),
        )
        .row(status_row(
            "settings_native_backend",
            snapshot.active_backend.to_string(),
        ))
        .row(status_row(
            "settings_permission",
            snapshot.permission.label(),
        ));
    if let Some(reason) = snapshot.degraded_reason.clone() {
        section = section.row(status_row("settings_degraded", reason));
    }
    if let Some(error) = snapshot.last_backend_error.clone() {
        section = section.row(status_row("settings_last_backend_error", error));
    }
    if let Some(caps) = snapshot.daemon_capabilities.clone() {
        section = section.row(status_row("settings_daemon_capabilities", caps));
    }
    section
        .row(super::super::button_row([
            Button::new(
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
            }),
            Button::new(
                "request-notification-permission",
                crate::i18n::localize("settings_request_permission"),
            )
            .variant(ButtonVariant::Outline)
            .disabled(!can_request_permission)
            .on_click(|_, _, cx| notifications::request_permission(cx)),
            Button::new(
                "open-notification-settings",
                crate::i18n::localize("settings_open_notification_settings"),
            )
            .variant(ButtonVariant::Outline)
            .disabled(!can_open_settings)
            .on_click(|_, _, cx| notifications::open_system_settings(cx)),
        ]))
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
        .row(status_row(
            "settings_push_notifications",
            crate::i18n::localize("settings_push_notifications_note"),
        ))
}
