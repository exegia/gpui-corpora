#[allow(unused_imports)]
use super::*;

#[derive(Clone)]
pub struct NativeNotificationState {
    pub service: Arc<NotificationService>,
    pub snapshot: NotificationRuntimeSnapshot,
}

impl Global for NativeNotificationState {}
