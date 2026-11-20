#[allow(unused_imports)]
use super::*;

use std::path::PathBuf;

/// Convert `input` into a Text-Fabric bundle beside it, as a background task.
///
/// Progress and the result appear as a toast keyed to the task. A new sidecar
/// runs for each conversion and is shut down when the job ends.
pub fn start_epub_conversion(input: PathBuf, cx: &mut App) {
    let id = TaskId::new();
    let name = input
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_default();
    tracing::info!(target: "corpora::tasks", task_id = %id, "starting EPUB conversion");
    start(
        id,
        crate::i18n::localize_with_args("tasks_epub_label", &[("name", &name)]),
        TaskProgress::Indeterminate,
        cx,
    );
    EpubConversionNotification::Running {
        name: &name,
        phase: None,
    }
    .push(id, cx);
    spawn_epub_conversion(id, input, name, cx);
}
