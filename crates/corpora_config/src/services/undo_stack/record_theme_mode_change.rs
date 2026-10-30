#[allow(unused_imports)]
use super::*;

pub fn record_theme_mode_change(before: ThemeMode, after: ThemeMode, cx: &mut App) {
    if before == after {
        return;
    }
    let state = cx.default_global::<UndoState>();
    let mut model = UndoModel::from_state(std::mem::take(state));
    model.record(UndoEntry {
        label: "Switch Theme".to_string(),
        undo_label: "Undo Theme Switch".to_string(),
        redo_label: "Redo Theme Switch".to_string(),
        created_at: AppTimestamp::now(),
        kind: UndoKind::ThemeMode { before, after },
    });
    *state = model.into_state();
    tracing::debug!(target: "corpora::undo", "recorded theme mode change");
}
