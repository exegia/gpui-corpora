#[allow(unused_imports)]
use super::*;

pub(crate) fn cancel(id: TaskId, reason: String, cx: &mut App) {
    mutate_task(id, cx, |task| {
        task.status = TaskStatus::Cancelled;
        task.finished_at = Some(AppTimestamp::now());
        task.error = Some(reason);
    });
}
