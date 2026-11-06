#[allow(unused_imports)]
use super::*;

#[derive(Clone, Debug)]
pub struct UndoEntry {
    pub label: String,
    pub undo_label: String,
    pub redo_label: String,
    pub created_at: AppTimestamp,
    pub kind: UndoKind,
}
