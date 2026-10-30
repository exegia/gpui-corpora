#[allow(unused_imports)]
use super::*;

pub(crate) fn runtime_boundaries_section() -> impl IntoElement {
    SettingsSection::new(crate::i18n::localize("settings_runtime_boundaries"))
        .row(super::super::button_row([
            outline("connectivity-check-now", "settings_check_connectivity_now")
                .on_click(|_, _, cx| connectivity::check_now(cx)),
            outline("session-sign-in", "settings_session_sign_in_demo").on_click(|_, _, cx| {
                session::set_state(SessionState::SigningIn, cx);
                session::set_state(
                    SessionState::SignedIn {
                        account_label: "demo-user".to_string(),
                    },
                    cx,
                );
            }),
            outline("session-sign-out", "settings_session_sign_out")
                .on_click(|_, _, cx| session::set_state(SessionState::SignedOut, cx)),
            outline("session-error-demo", "settings_session_error_demo").on_click(|_, _, cx| {
                session::set_state(SessionState::Error("demo session error".to_string()), cx);
            }),
        ]))
        .row(super::super::button_row([
            outline(
                "secure-storage-write-demo",
                "settings_write_secure_value_demo",
            )
            .on_click(|_, _, cx| {
                let message =
                    match secure_storage::set_secret("corpora", "demo-token", "demo-value", cx) {
                        Ok(()) => crate::i18n::localize("settings_secure_value_written"),
                        Err(err) => format!(
                            "{}: {err}",
                            crate::i18n::localize("settings_secure_write_failed")
                        ),
                    };
                toast(message, cx);
            }),
            outline(
                "secure-storage-read-demo",
                "settings_read_secure_value_demo",
            )
            .on_click(|_, _, cx| {
                let message = match secure_storage::get_secret("corpora", "demo-token", cx) {
                    Ok(Some(_)) => crate::i18n::localize("settings_secure_value_exists"),
                    Ok(None) => crate::i18n::localize("settings_secure_value_missing"),
                    Err(err) => format!(
                        "{}: {err}",
                        crate::i18n::localize("settings_secure_read_failed")
                    ),
                };
                toast(message, cx);
            }),
            outline(
                "secure-storage-delete-demo",
                "settings_delete_secure_value_demo",
            )
            .on_click(|_, _, cx| {
                let message = match secure_storage::delete_secret("corpora", "demo-token", cx) {
                    Ok(()) => crate::i18n::localize("settings_secure_value_deleted"),
                    Err(err) => format!(
                        "{}: {err}",
                        crate::i18n::localize("settings_secure_delete_failed")
                    ),
                };
                toast(message, cx);
            }),
        ]))
}
