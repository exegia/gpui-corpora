#[allow(unused_imports)]
use super::*;

/// The open workspace, for callers with no window (global hotkey, IPC).
pub(crate) struct WorkspaceHandle(pub(crate) WeakEntity<Workspace>);

impl Global for WorkspaceHandle {}

pub(crate) fn current_workspace(cx: &App) -> Option<WeakEntity<Workspace>> {
    cx.try_global::<WorkspaceHandle>()
        .map(|handle| handle.0.clone())
}
