#[allow(unused_imports)]
use super::*;

pub fn undo(cx: &mut App) -> bool {
    // The intermediate store publishes `applying = true` so a reentrant
    // `record` during apply is suppressed; the apply itself clears it.
    let state = cx.default_global::<UndoState>();
    let mut model = UndoModel::from_state(std::mem::take(state));
    let Some(entry) = model.pop_undo() else {
        *state = model.into_state();
        return false;
    };
    model.applying = true;
    *state = model.into_state();
    apply_inverse(&entry.kind, cx);
    let state = cx.default_global::<UndoState>();
    let mut model = UndoModel::from_state(std::mem::take(state));
    model.applying = false;
    model.push_redo(entry);
    *state = model.into_state();
    true
}
