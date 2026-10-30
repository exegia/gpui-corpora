#[allow(unused_imports)]
use super::*;

pub(crate) fn mutate_snapshot(cx: &mut App, f: impl FnOnce(&mut NotificationRuntimeSnapshot)) {
    cx.update_global::<NativeNotificationState, _>(|state, cx| {
        f(&mut state.snapshot);
        cx.refresh_windows();
    });
}
