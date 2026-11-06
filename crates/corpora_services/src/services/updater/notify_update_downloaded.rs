#[allow(unused_imports)]
use super::*;

pub(crate) fn notify_update_downloaded(version: &str, cx: &mut gpui::App) {
    dispatch_background_notification(
        &format!(
            "{} v{version} {}",
            crate::i18n::localize("updates_title_prefix"),
            crate::i18n::localize("updates_ready_suffix")
        ),
        &crate::i18n::localize("updates_ready_body"),
        cx,
    );
}
