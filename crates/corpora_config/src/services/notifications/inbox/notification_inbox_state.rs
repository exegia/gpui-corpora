#[allow(unused_imports)]
use super::*;

#[derive(Clone, Debug, Default)]
pub struct NotificationInboxState {
    pub items: Vec<NotificationInboxItem>,
}

impl Global for NotificationInboxState {}
