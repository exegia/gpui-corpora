#[allow(unused_imports)]
use super::*;

pub fn can_undo(cx: &App) -> Option<String> {
    cx.try_global::<UndoState>()
        .and_then(|state| state.past.last())
        .map(|entry| entry.undo_label.clone())
}
