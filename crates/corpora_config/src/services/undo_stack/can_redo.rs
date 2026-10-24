#[allow(unused_imports)]
use super::*;

pub fn can_redo(cx: &App) -> Option<String> {
    cx.try_global::<UndoState>()
        .and_then(|state| state.future.last())
        .map(|entry| entry.redo_label.clone())
}
