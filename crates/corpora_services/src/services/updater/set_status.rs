#[allow(unused_imports)]
use super::*;

pub(crate) fn set_status(status: UpdateStatus, cx: &mut gpui::App) {
    tracing::debug!(
        target: "corpora::updater",
        status = ?status,
        "update status changed"
    );
    UpdateSnapshot::update_global(cx, |snap, _cx| {
        snap.status = status;
    });
}
