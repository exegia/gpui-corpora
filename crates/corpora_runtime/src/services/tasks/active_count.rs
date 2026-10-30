#[allow(unused_imports)]
use super::*;

pub fn active_count(cx: &App) -> usize {
    cx.try_global::<TaskRegistry>()
        .map(|state| {
            state
                .tasks
                .iter()
                .filter(|task| matches!(task.status, TaskStatus::Queued | TaskStatus::Running))
                .count()
        })
        .unwrap_or(0)
}
