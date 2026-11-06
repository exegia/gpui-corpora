#[allow(unused_imports)]
use super::*;

#[cfg(target_os = "macos")]
pub(crate) async fn platform_permission_state() -> NotificationPermissionState {
    use std::{cell::RefCell, ptr::NonNull};

    use block2::RcBlock;
    use objc2_user_notifications::{
        UNAuthorizationStatus, UNNotificationSettings, UNUserNotificationCenter,
    };

    let (tx, rx) = tokio::sync::oneshot::channel::<NotificationPermissionState>();

    // SAFETY: Apple invokes the completion handler exactly once on the main
    // thread with a valid pointer; `tx.take()` prevents double-send.
    unsafe {
        let tx = RefCell::new(Some(tx));
        let block = RcBlock::new(move |settings: NonNull<UNNotificationSettings>| {
            let status = settings.as_ref().authorizationStatus();
            tracing::debug!(target: LOG, ?status, "received macOS notification authorization status");
            let state = match status {
                UNAuthorizationStatus::Authorized
                | UNAuthorizationStatus::Provisional
                | UNAuthorizationStatus::Ephemeral => NotificationPermissionState::Authorized,
                UNAuthorizationStatus::Denied => NotificationPermissionState::Denied,
                UNAuthorizationStatus::NotDetermined => NotificationPermissionState::NotDetermined,
                status => NotificationPermissionState::Unavailable(format!(
                    "unknown macOS authorization status: {status:?}"
                )),
            };

            if let Some(tx) = tx.take()
                && tx.send(state).is_err()
            {
                tracing::warn!(target: LOG, "permission state receiver dropped");
            }
        });

        UNUserNotificationCenter::currentNotificationCenter()
            .getNotificationSettingsWithCompletionHandler(&block);
    }

    rx.await
        .unwrap_or_else(|err| NotificationPermissionState::Unavailable(err.to_string()))
}

#[cfg(not(target_os = "macos"))]
pub(crate) async fn platform_permission_state() -> NotificationPermissionState {
    NotificationPermissionState::Unsupported
}
