#[allow(unused_imports)]
use super::*;

pub(crate) fn fail(id: TaskId, error: String, cx: &mut App) {
    mutate_task(id, cx, |task| {
        task.status = TaskStatus::Failed;
        task.finished_at = Some(AppTimestamp::now());
        task.error = Some(error);
    });
}
