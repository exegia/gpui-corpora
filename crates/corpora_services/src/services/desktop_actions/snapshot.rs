#[allow(unused_imports)]
use super::*;

pub fn snapshot(cx: &App) -> DesktopActionsSnapshot {
    cx.try_global::<DesktopActionsState>()
        .map(|s| s.snapshot.clone())
        .unwrap_or_default()
}
