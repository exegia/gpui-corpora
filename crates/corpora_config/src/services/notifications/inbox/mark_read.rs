use super::*;

/// Mark one notification read in the live inbox and persisted configuration.
pub fn mark_read(id: NotificationId, cx: &mut App) {
    let state = cx.default_global::<NotificationInboxState>();
    let Some(item) = state.items.iter_mut().find(|item| item.id == id) else {
        return;
    };
    if item.read {
        return;
    }
    item.read = true;
    app_state::update_config(cx, |config| {
        if let Some(item) = config
            .notification_inbox
            .iter_mut()
            .find(|item| item.id == id)
        {
            item.read = true;
        }
    });
}
