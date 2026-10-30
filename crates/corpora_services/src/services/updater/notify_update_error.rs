#[allow(unused_imports)]
use super::*;

/// Permanent update errors only (retries exhausted).
pub(crate) fn notify_update_error(cx: &mut gpui::App) {
    dispatch_background_notification(
        &crate::i18n::localize("updates_error_title"),
        &crate::i18n::localize("updates_error_body"),
        cx,
    );
}
