#[allow(unused_imports)]
use super::*;

pub fn bar(cx: &App) -> StatusBar {
    let connectivity = match connectivity::snapshot(cx).state {
        ConnectivityState::Online => Connectivity::Online,
        ConnectivityState::Unknown => Connectivity::Reconnecting,
        ConnectivityState::Offline | ConnectivityState::CaptiveOrFiltered => Connectivity::Offline,
    };
    let session = match session::snapshot(cx).state {
        SessionState::SignedOut => "Not signed in".to_string(),
        SessionState::SigningIn => "Signing in…".to_string(),
        SessionState::SignedIn { account_label } => format!("Signed in as {account_label}"),
        SessionState::Error(error) => format!("Session error: {error}"),
    };
    let unread = crate::notifications::inbox::unread_count(cx);
    let tasks = crate::tasks::active_count(cx);

    let mut bar = StatusBar::new()
        .left(
            StatusBarItem::new("workspace-mode")
                .icon(IconName::Database)
                .label("Local workspace"),
        )
        .left(OfflineIndicator::new("connectivity", connectivity));
    if tasks > 0 {
        bar = bar.left(
            StatusBarItem::new("tasks")
                .icon(IconName::Activity)
                .label(format!("{tasks} running")),
        );
    }
    if let Some(label) = update_label(cx) {
        bar = bar.left(
            StatusBarItem::new("updater")
                .icon(IconName::Download)
                .label(label),
        );
    }
    if let Some(frame) = frame_time_label(cx) {
        bar = bar.right(StatusBarItem::new("frame-time").label(frame));
    }
    bar.right(navigate_on_click(
        StatusBarItem::new("notifications")
            .icon(IconName::Bell)
            .label(unread.to_string())
            .tooltip("Notifications"),
        navigation::NOTIFICATIONS,
    ))
    .right(
        StatusBarItem::new("auth-status")
            .icon(IconName::Shield)
            .label(session),
    )
}
