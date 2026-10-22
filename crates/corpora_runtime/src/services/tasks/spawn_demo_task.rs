#[allow(unused_imports)]
use super::*;

pub(crate) fn spawn_demo_task(id: TaskId, cx: &mut App) {
    cx.spawn(async move |cx| {
        let background = cx.background_executor();
        for step in [20_u8, 40, 60, 80, 100] {
            if is_shutdown_requested() {
                cx.update(|cx| {
                    fail(id, "cancelled during shutdown".to_string(), cx);
                    push_demo_task_notification(id, DemoTaskNotificationKind::Cancelled, cx);
                });
                return;
            }
            background.timer(Duration::from_millis(350)).await;
            cx.update(move |cx| {
                tracing::debug!(
                    target: "corpora::tasks",
                    task_id = %id,
                    progress = step,
                    "updating demo task progress"
                );
                update_progress(id, TaskProgress::Percent(step), cx);
            });
        }
        cx.update(move |cx| {
            tracing::info!(
                target: "corpora::tasks",
                task_id = %id,
                "demo task succeeded"
            );
            succeed(id, cx);
            push_demo_task_notification(id, DemoTaskNotificationKind::Success, cx);
        });
    })
    .detach();
}
