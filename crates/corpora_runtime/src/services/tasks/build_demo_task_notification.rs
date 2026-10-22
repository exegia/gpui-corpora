#[allow(unused_imports)]
use super::*;

/// Keyed per task so re-pushing replaces rather than stacks.
pub(crate) fn build_demo_task_notification(kind: DemoTaskNotificationKind) -> Toast {
    match kind {
        DemoTaskNotificationKind::Loading => {
            Toast::new(crate::i18n::localize("tasks_demo_started_title"))
                .body(crate::i18n::localize("tasks_demo_running_body"))
                .severity(Severity::Info)
                .stay(None)
        }
        DemoTaskNotificationKind::Success => {
            Toast::new(crate::i18n::localize("tasks_demo_completed_title"))
                .body(crate::i18n::localize("tasks_demo_succeeded_body"))
                .severity(Severity::Success)
        }
        DemoTaskNotificationKind::Cancelled => {
            Toast::new(crate::i18n::localize("tasks_demo_cancelled_title"))
                .body(crate::i18n::localize("tasks_demo_cancelled_body"))
                .severity(Severity::Warning)
                .stay(None)
        }
    }
}
