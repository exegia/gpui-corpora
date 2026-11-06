#[allow(unused_imports)]
use super::*;

pub fn initialize(cx: &mut App) {
    let config = app_state::config(cx);
    cx.set_global(NotificationInboxState {
        items: config.notification_inbox,
    });
}
