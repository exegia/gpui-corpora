#[allow(unused_imports)]
use super::*;

pub(crate) fn update_result(action: &str, error: Option<String>, cx: &mut App) {
    let Some(state) = cx.try_global::<DesktopActionsState>() else {
        return;
    };
    let active_watchers = state
        .inner
        .lock()
        .ok()
        .map(|inner| inner.watchers.len())
        .unwrap_or(0);
    cx.update_global::<DesktopActionsState, _>(|state, _cx| {
        state.snapshot.last_error = error.clone();
        state.snapshot.active_watchers = active_watchers;
    });
    if let Some(error) = error {
        tracing::warn!(
            target: "corpora::desktop_actions",
            action,
            error = %error,
            "desktop action failed"
        );
        crate::error_surface::report(
            crate::i18n::localize_with_args(
                "desktop_action_failed",
                &[("action", action), ("error", error.as_str())],
            ),
            crate::errors::AppErrorSeverity::Warning,
            crate::error_surface::ErrorCategory::System,
            vec![
                crate::error_surface::ErrorAction::Retry,
                crate::error_surface::ErrorAction::Dismiss,
            ],
            cx,
        );
    } else {
        tracing::debug!(
            target: "corpora::desktop_actions",
            action,
            "desktop action succeeded"
        );
    }
}
