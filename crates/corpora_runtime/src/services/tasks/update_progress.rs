#[allow(unused_imports)]
use super::*;

pub(crate) fn update_progress(id: TaskId, progress: TaskProgress, cx: &mut App) {
    mutate_task(id, cx, |task| {
        task.progress = progress;
        task.status = TaskStatus::Running;
    });
}
