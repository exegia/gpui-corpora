#[allow(unused_imports)]
use super::*;

pub(crate) fn push_demo_task_notification(
    id: TaskId,
    kind: DemoTaskNotificationKind,
    cx: &mut App,
) {
    crate::ui::toasts::replace(
        format!("demo-task-{id}"),
        build_demo_task_notification(kind),
        cx,
    );
}
