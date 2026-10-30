#[allow(unused_imports)]
use super::*;

pub(crate) fn sample_entry() -> UndoEntry {
    UndoEntry {
        label: "Switch Theme".to_string(),
        undo_label: "Undo Theme Switch".to_string(),
        redo_label: "Redo Theme Switch".to_string(),
        created_at: AppTimestamp::now(),
        kind: UndoKind::ThemeMode {
            before: ThemeMode::Light,
            after: ThemeMode::Dark,
        },
    }
}
