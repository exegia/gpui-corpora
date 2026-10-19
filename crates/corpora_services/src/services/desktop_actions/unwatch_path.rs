#[allow(unused_imports)]
use super::*;

pub(crate) fn unwatch_path(id: u64, cx: &mut App) -> bool {
    let Some(state) = cx.try_global::<DesktopActionsState>() else {
        return false;
    };
    let (removed, active_watchers) = if let Ok(mut inner) = state.inner.lock() {
        let removed = inner.watchers.remove(&id).is_some();
        (removed, inner.watchers.len())
    } else {
        return false;
    };
    cx.update_global::<DesktopActionsState, _>(|state, _cx| {
        state.snapshot.active_watchers = active_watchers;
    });
    removed
}
