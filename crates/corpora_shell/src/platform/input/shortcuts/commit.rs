#[allow(unused_imports)]
use super::*;

pub(crate) fn commit(state: ShortcutState, cx: &mut App) {
    set_capability(&state, cx);
    cx.update_global::<ShortcutState, _>(|s, _cx| {
        *s = state;
    });
}
