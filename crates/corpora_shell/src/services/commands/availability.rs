#[allow(unused_imports)]
use super::*;

pub fn availability(id: CommandId, cx: &App) -> CommandAvailability {
    let desktop = crate::desktop_actions::snapshot(cx);
    match id {
        CommandId::OpenHome
        | CommandId::OpenSettings
        | CommandId::OpenNotifications
        | CommandId::OpenDiagnostics
        | CommandId::OpenAbout
        | CommandId::ThemeLight
        | CommandId::ThemeDark
        | CommandId::StartDemoTask => CommandAvailability {
            enabled: true,
            disabled_reason: None,
        },
        CommandId::CheckConnectivity => CommandAvailability {
            enabled: true,
            disabled_reason: None,
        },
        CommandId::CopyDiagnostics => CommandAvailability {
            enabled: desktop.clipboard_available,
            disabled_reason: disabled_reason(
                "command_reason_clipboard_unavailable",
                !desktop.clipboard_available,
            ),
        },
        CommandId::OpenLogsFolder | CommandId::OpenConfigFolder => CommandAvailability {
            enabled: desktop.opener_available,
            disabled_reason: disabled_reason(
                "command_reason_opener_unavailable",
                !desktop.opener_available,
            ),
        },
        CommandId::Undo => {
            let undo = crate::undo_stack::can_undo(cx);
            CommandAvailability {
                enabled: undo.is_some(),
                disabled_reason: disabled_reason("command_reason_no_undo", undo.is_none()),
            }
        }
        CommandId::Redo => {
            let redo = crate::undo_stack::can_redo(cx);
            CommandAvailability {
                enabled: redo.is_some(),
                disabled_reason: disabled_reason("command_reason_no_redo", redo.is_none()),
            }
        }
        CommandId::Restart => CommandAvailability {
            enabled: cfg!(any(unix, windows)),
            disabled_reason: disabled_reason(
                "command_reason_restart_unavailable",
                !cfg!(any(unix, windows)),
            ),
        },
    }
}
