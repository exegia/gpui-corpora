#[allow(unused_imports)]
use super::*;

pub fn snapshot(cx: &App) -> NotificationRuntimeSnapshot {
    cx.global::<NativeNotificationState>().snapshot.clone()
}
