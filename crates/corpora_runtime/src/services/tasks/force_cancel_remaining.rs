#[allow(unused_imports)]
use super::*;

pub(crate) fn force_cancel_remaining(cx: &mut App) {
    let registry = cx.default_global::<TaskRegistry>();
    let mut changed = false;
    for task in &mut registry.tasks {
        if matches!(task.status, TaskStatus::Queued | TaskStatus::Running) {
            task.status = TaskStatus::Cancelled;
            task.finished_at = Some(AppTimestamp::now());
            task.error = Some("cancelled during app shutdown (drain timeout)".to_string());
            changed = true;
        }
    }
    if changed {
        tracing::info!(
            target: "corpora::tasks",
            "force-cancelled remaining tasks after drain timeout"
        );
    }
}
