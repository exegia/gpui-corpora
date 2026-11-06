#[allow(unused_imports)]
use super::*;

pub fn snapshot(cx: &App) -> Vec<NotificationInboxItem> {
    cx.try_global::<NotificationInboxState>()
        .map(|state| state.items.clone())
        .unwrap_or_default()
}
