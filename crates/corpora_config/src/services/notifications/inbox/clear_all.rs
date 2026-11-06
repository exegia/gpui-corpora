#[allow(unused_imports)]
use super::*;

pub fn clear_all(cx: &mut App) {
    {
        let state = cx.default_global::<NotificationInboxState>();
        state.items.clear();
    }
    app_state::update_config(cx, |config| {
        config.notification_inbox = Vec::new();
    });
}
