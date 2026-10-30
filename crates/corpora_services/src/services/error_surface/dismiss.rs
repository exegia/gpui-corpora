#[allow(unused_imports)]
use super::*;

pub fn dismiss(id: EventId, cx: &mut App) {
    cx.update_global::<ErrorSurfaceState, _>(|state, _cx| {
        state.records.retain(|r| r.id != id);
    });
}
