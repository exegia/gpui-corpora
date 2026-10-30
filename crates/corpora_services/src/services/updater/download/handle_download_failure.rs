#[allow(unused_imports)]
use super::*;

pub(crate) fn handle_download_failure(error: String, cx: &mut App) {
    let scheduled = super::super::check::schedule_retry(
        cx,
        super::super::check::download_retry_field,
        "download",
        download_update,
    );
    if scheduled {
        // Back to Available so the UI can re-attempt.
        UpdateSnapshot::update_global(cx, |snap, _cx| {
            snap.status = UpdateStatus::Available {
                version: String::new(),
                notes: String::new(),
            };
        });
    } else {
        tracing::error!(
            target: "corpora::updater",
            error = %error,
            "download failed — retries exhausted"
        );
        super::super::set_status(UpdateStatus::Error(error), cx);
        super::super::notify_update_error(cx);
    }
}
