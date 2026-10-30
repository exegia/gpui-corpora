#[allow(unused_imports)]
use super::*;

pub(crate) fn start(id: TaskId, label: String, progress: TaskProgress, cx: &mut App) {
    tracing::info!(
        target: "corpora::tasks",
        task_id = %id,
        task_label = %label,
        progress = ?progress,
        active_tasks_before = active_count(cx),
        "background task starting"
    );
    cx.default_global::<TaskRegistry>().tasks.insert(
        0,
        BackgroundTask {
            id,
            label,
            status: TaskStatus::Running,
            progress,
            started_at: AppTimestamp::now(),
            finished_at: None,
            error: None,
        },
    );
    tracing::debug!(
        target: "corpora::tasks",
        task_id = %id,
        active_tasks = active_count(cx),
        "background task registered"
    );
}
