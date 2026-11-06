#[allow(unused_imports)]
use super::*;

pub fn record(item: NotificationInboxItem, cx: &mut App) {
    // PERF: Clone the item before inserting into state so update_config can use it
    // without re-reading from global (avoids borrowing global + config simultaneously).
    let item_for_config = item.clone();
    {
        let state = cx.default_global::<NotificationInboxState>();
        state.items.insert(0, item);
        if state.items.len() > MAX_INBOX_ITEMS {
            state.items.truncate(MAX_INBOX_ITEMS);
        }
    }
    app_state::update_config(cx, |config| {
        config.notification_inbox.insert(0, item_for_config);
        config.notification_inbox.truncate(MAX_INBOX_ITEMS);
    });
}
