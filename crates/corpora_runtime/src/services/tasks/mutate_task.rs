#[allow(unused_imports)]
use super::*;

pub(crate) fn mutate_task(id: TaskId, cx: &mut App, mutate: impl FnOnce(&mut BackgroundTask)) {
    let registry = cx.default_global::<TaskRegistry>();
    if let Some(task) = registry.tasks.iter_mut().find(|task| task.id == id) {
        mutate(task);
        tracing::debug!(
            target: "corpora::tasks",
            task_id = %id,
            status = ?task.status,
            progress = ?task.progress,
            finished = task.finished_at.is_some(),
            "background task mutated"
        );
    }
}
