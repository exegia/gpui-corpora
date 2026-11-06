#[allow(unused_imports)]
use super::*;

pub(crate) fn watch_path(path: PathBuf, cx: &mut App) -> Result<u64, DesktopActionError> {
    let state = cx
        .try_global::<DesktopActionsState>()
        .ok_or(DesktopActionError::Unavailable)?;
    let mut inner = state
        .inner
        .lock()
        .map_err(|_| DesktopActionError::LockPoisoned)?;
    let watcher_id = inner.next_watcher_id;
    inner.next_watcher_id += 1;

    let mut watcher =
        notify::recommended_watcher(move |result: notify::Result<notify::Event>| match result {
            Ok(event) => tracing::debug!(
                target: "corpora::desktop_actions",
                watcher_id,
                kind = ?event.kind,
                paths = ?event.paths,
                "watch event"
            ),
            Err(err) => tracing::warn!(
                target: "corpora::desktop_actions",
                watcher_id,
                error = %err,
                "watch event error"
            ),
        })
        .map_err(DesktopActionError::from)?;
    watcher
        .watch(&path, RecursiveMode::NonRecursive)
        .map_err(DesktopActionError::from)?;
    inner.watchers.insert(watcher_id, watcher);
    let active_watchers = inner.watchers.len();
    drop(inner);
    cx.update_global::<DesktopActionsState, _>(|state, _cx| {
        state.snapshot.active_watchers = active_watchers;
        state.snapshot.last_error = None;
    });
    tracing::info!(target: "corpora::desktop_actions", watcher_id, path = %path.display(), "watcher registered");
    Ok(watcher_id)
}
