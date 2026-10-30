#[allow(unused_imports)]
use super::*;

pub fn execute(id: CommandId, cx: &mut App) {
    match id {
        CommandId::OpenHome => navigate(navigation::HOME, cx),
        CommandId::OpenSettings => {
            cx.defer(|cx| {
                if let Err(error) = crate::app::window::open_settings_window(None, cx) {
                    tracing::error!(%error, "could not open Settings window");
                }
            });
        }
        CommandId::OpenNotifications => navigate(navigation::NOTIFICATIONS, cx),
        CommandId::OpenDiagnostics => navigate(navigation::DIAGNOSTICS, cx),
        CommandId::OpenAbout => navigate(navigation::ABOUT, cx),
        CommandId::ThemeLight => crate::app::set_theme_mode(ThemeMode::Light, cx),
        CommandId::ThemeDark => crate::app::set_theme_mode(ThemeMode::Dark, cx),
        CommandId::StartDemoTask => crate::tasks::start_demo_task(cx),
        CommandId::CheckConnectivity => crate::connectivity::check_now(cx),
        CommandId::CopyDiagnostics => {
            if let Err(error) = crate::desktop_actions::copy_diagnostics(cx) {
                tracing::warn!(target: "corpora::commands", %error, "copy diagnostics failed");
                report_error(
                    format!(
                        "{}: {error}",
                        crate::i18n::localize("command_copy_diagnostics_failed")
                    ),
                    crate::error_surface::ErrorCategory::System,
                    vec![
                        crate::error_surface::ErrorAction::Retry,
                        crate::error_surface::ErrorAction::Dismiss,
                    ],
                    cx,
                );
            }
        }
        CommandId::OpenLogsFolder => {
            if let Err(error) = crate::desktop_actions::open_logs_folder(cx) {
                tracing::warn!(target: "corpora::commands", %error, "open logs folder failed");
                report_error(
                    format!(
                        "{}: {error}",
                        crate::i18n::localize("command_open_logs_failed")
                    ),
                    crate::error_surface::ErrorCategory::Storage,
                    vec![
                        crate::error_surface::ErrorAction::OpenSettings,
                        crate::error_surface::ErrorAction::Dismiss,
                    ],
                    cx,
                );
            }
        }
        CommandId::OpenConfigFolder => {
            if let Err(error) = crate::desktop_actions::open_config_folder(cx) {
                tracing::warn!(target: "corpora::commands", %error, "open config folder failed");
                report_error(
                    format!(
                        "{}: {error}",
                        crate::i18n::localize("command_open_config_failed")
                    ),
                    crate::error_surface::ErrorCategory::Config,
                    vec![
                        crate::error_surface::ErrorAction::OpenSettings,
                        crate::error_surface::ErrorAction::Dismiss,
                    ],
                    cx,
                );
            }
        }
        CommandId::Undo => {
            let _ = crate::undo_stack::undo(cx);
        }
        CommandId::Redo => {
            let _ = crate::undo_stack::redo(cx);
        }
        CommandId::Restart => {
            #[cfg(any(unix, windows))]
            cx.dispatch_action(&crate::app::Restart);
            #[cfg(not(any(unix, windows)))]
            {
                let _ = cx;
                tracing::warn!(
                    target: "corpora::commands",
                    "restart command unavailable on this platform"
                );
            }
        }
    }
}
