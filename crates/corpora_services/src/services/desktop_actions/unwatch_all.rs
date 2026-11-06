#[allow(unused_imports)]
use super::*;

pub fn unwatch_all(cx: &mut App) -> usize {
    let state = match cx.try_global::<DesktopActionsState>().cloned() {
        Some(state) => state,
        None => return 0,
    };
    let watcher_ids = state
        .inner
        .lock()
        .ok()
        .map(|inner| inner.watchers.keys().copied().collect::<Vec<_>>())
        .unwrap_or_default();
    let count = watcher_ids.len();
    for watcher_id in watcher_ids {
        let _ = unwatch_path(watcher_id, cx);
    }
    count
}
