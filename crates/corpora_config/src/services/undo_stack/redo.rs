#[allow(unused_imports)]
use super::*;

pub fn redo(cx: &mut App) -> bool {
    let state = cx.default_global::<UndoState>();
    let mut model = UndoModel::from_state(std::mem::take(state));
    let Some(entry) = model.pop_redo() else {
        *state = model.into_state();
        return false;
    };
    model.applying = true;
    *state = model.into_state();
    apply_forward(&entry.kind, cx);
    let state = cx.default_global::<UndoState>();
    let mut model = UndoModel::from_state(std::mem::take(state));
    model.applying = false;
    model.push_undo(entry);
    *state = model.into_state();
    true
}
