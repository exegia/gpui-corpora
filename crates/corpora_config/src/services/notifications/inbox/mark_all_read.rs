#[allow(unused_imports)]
use super::*;

pub fn mark_all_read(cx: &mut App) {
    {
        let state = cx.default_global::<NotificationInboxState>();
        for item in &mut state.items {
            item.read = true;
        }
    }
    // PERF: Update config in-place instead of cloning the entire items Vec.
    app_state::update_config(cx, |config| {
        for item in &mut config.notification_inbox {
            item.read = true;
        }
    });
}
