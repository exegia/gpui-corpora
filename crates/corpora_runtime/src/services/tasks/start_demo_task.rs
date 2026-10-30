#[allow(unused_imports)]
use super::*;

pub fn start_demo_task(cx: &mut App) {
    let id = TaskId::new();
    tracing::info!(
        target: "corpora::tasks",
        task_id = %id,
        "starting demo task"
    );
    start(
        id,
        crate::i18n::localize("tasks_demo_label"),
        TaskProgress::Percent(0),
        cx,
    );
    push_demo_task_notification(id, DemoTaskNotificationKind::Loading, cx);
    spawn_demo_task(id, cx);
}
