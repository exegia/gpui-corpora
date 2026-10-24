#[allow(unused_imports)]
use super::*;

pub(crate) fn notify_update_available(version: &str, cx: &mut gpui::App) {
    dispatch_background_notification(
        &format!(
            "{} v{version} {}",
            crate::i18n::localize("updates_title_prefix"),
            crate::i18n::localize("updates_available_suffix")
        ),
        &crate::i18n::localize("updates_available_body"),
        cx,
    );
}
