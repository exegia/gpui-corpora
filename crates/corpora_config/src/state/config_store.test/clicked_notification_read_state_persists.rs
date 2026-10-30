use super::*;
use crate::{
    ids::NotificationId,
    notifications::inbox::{
        self, NotificationInboxItem, NotificationInboxKind, NotificationInboxState,
    },
    time::AppTimestamp,
};

#[gpui::test]
async fn clicked_notification_read_state_persists(cx: &TestAppContext) {
    let _guard = debounce_test_lock();
    let dir = tempdir().unwrap();
    let state_file = dir.path().join("state.json");
    let first = NotificationInboxItem {
        id: NotificationId::new(),
        created_at: AppTimestamp::now(),
        title: "First".into(),
        body: "Message".into(),
        read: false,
        backend: "in-app only".into(),
        delivered_natively: false,
        degraded: false,
        error_summary: None,
        kind: NotificationInboxKind::SettingsUpdate,
    };
    let second = NotificationInboxItem {
        id: NotificationId::new(),
        ..first.clone()
    };
    let items = vec![first.clone(), second];
    cx.update(|cx| {
        let mut state = clean_state(&state_file);
        state.config.notification_inbox = items.clone();
        cx.set_global(state);
        cx.set_global(NotificationInboxState { items });
        inbox::mark_read(NotificationId::new(), cx);
        assert_eq!(inbox::unread_count(cx), 2);
        inbox::mark_read(first.id, cx);
        assert_eq!(inbox::unread_count(cx), 1);
        assert!(inbox::snapshot(cx)[0].read);
        assert!(!inbox::snapshot(cx)[1].read);
        force_save(cx);
    });
    let loaded = read_persisted(&state_file);
    assert!(loaded.notification_inbox[0].read);
    assert!(!loaded.notification_inbox[1].read);
    cx.update(|cx| {
        inbox::mark_read(first.id, cx);
        assert_eq!(inbox::unread_count(cx), 1);
        assert!(!cx.global::<AppState>().dirty);
    });
}
