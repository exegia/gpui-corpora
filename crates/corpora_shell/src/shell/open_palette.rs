#[allow(unused_imports)]
use super::*;

/// Toggle the command palette from outside the window (global hotkey).
pub fn open_palette(cx: &mut App) {
    let Some(workspace) = cx.try_global::<WorkspaceHandle>().map(|h| h.0.clone()) else {
        return;
    };
    cx.activate(true);
    workspace
        .update(cx, |workspace, cx| workspace.toggle_palette(cx))
        .ok();
}
