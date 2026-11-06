#[allow(unused_imports)]
use super::*;

/// Returns the number of unread inbox items without cloning the inbox.
pub fn unread_count(cx: &App) -> usize {
    cx.try_global::<NotificationInboxState>()
        .map(|state| state.items.iter().filter(|item| !item.read).count())
        .unwrap_or(0)
}
