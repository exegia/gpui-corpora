#[allow(unused_imports)]
use super::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AppEventKind {
    /// A gpui-router path such as `/settings`.
    Navigate(String),
    DeepLinkReceived(String),
    /// Typed command forwarded from a second instance over IPC; the dispatch
    /// layer translates it into app actions.
    RemoteCommand(ForwardedCommand),
    AppError {
        message: String,
        severity: crate::errors::AppErrorSeverity,
    },
    DiagnosticsChanged,
    Test {
        message: String,
    },
}
