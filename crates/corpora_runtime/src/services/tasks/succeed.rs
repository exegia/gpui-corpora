#[allow(unused_imports)]
use super::*;

pub(crate) fn succeed(id: TaskId, cx: &mut App) {
    mutate_task(id, cx, |task| {
        task.status = TaskStatus::Succeeded;
        task.progress = TaskProgress::Percent(100);
        task.finished_at = Some(AppTimestamp::now());
        task.error = None;
    });
}
