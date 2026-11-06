#[allow(unused_imports)]
use super::*;

pub(crate) fn handle_check_failure(error: String, cx: &mut App) {
    let scheduled = schedule_retry(cx, check_retry_field, "check", check_for_updates);
    super::super::set_status(UpdateStatus::Error(error.clone()), cx);
    if !scheduled {
        tracing::error!(
            target: "corpora::updater",
            error = %error,
            "update check failed — retries exhausted, setting permanent error"
        );
        super::super::notify_update_error(cx);
    }
}
