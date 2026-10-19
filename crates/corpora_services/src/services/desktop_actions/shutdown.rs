#[allow(unused_imports)]
use super::*;

pub fn shutdown(cx: &mut App) {
    let Some(state) = cx.try_global::<DesktopActionsState>() else {
        return;
    };
    if let Ok(mut inner) = state.inner.lock() {
        inner.watchers.clear();
    }
    cx.update_global::<DesktopActionsState, _>(|state, _cx| {
        state.snapshot.active_watchers = 0;
    });
}
