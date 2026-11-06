#[allow(unused_imports)]
use super::*;

pub(crate) fn pick_with_dialog(
    action: &'static str,
    file: Option<PathBuf>,
    cx: &mut App,
) -> Option<PathBuf> {
    tracing::info!(target: "corpora::desktop_actions", file = ?file, "file picker result");
    update_result(action, None, cx);
    file
}
