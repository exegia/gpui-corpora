#[allow(unused_imports)]
use super::*;

pub fn refresh_permission_state(cx: &mut App) {
    let service = cx.global::<NativeNotificationState>().service.clone();
    tracing::debug!(target: LOG, "scheduling async permission refresh");
    cx.spawn(async move |cx| {
        let permission = service.refresh_permission_state().await;
        tracing::info!(target: LOG, ?permission, "permission refresh completed");
        cx.update(move |cx| {
            mutate_snapshot(cx, |snapshot| {
                snapshot.permission = permission;
            });
        });
    })
    .detach();
}
