#[allow(unused_imports)]
use super::*;

pub(crate) fn actions_xml(request: &NotificationRequest) -> String {
    match request.category.as_deref() {
        Some(CATEGORY_ACTIONS) => format!(
            r#"<actions><action content="{}" arguments="{ACTION_OPEN}" activationType="foreground"/><action content="{}" arguments="{ACTION_SNOOZE}" activationType="foreground"/></actions>"#,
            escaped(&crate::i18n::localize("toast_open")),
            escaped(&crate::i18n::localize("toast_snooze")),
        ),
        Some(CATEGORY_REPLY) => format!(
            r#"<actions><input id="{REPLY_INPUT_ID}" type="text" placeHolderContent="{}"/><action content="{}" arguments="{ACTION_REPLY}" hint-inputId="{REPLY_INPUT_ID}"/></actions>"#,
            escaped(&crate::i18n::localize("toast_reply_placeholder")),
            escaped(&crate::i18n::localize("toast_send")),
        ),
        _ => String::new(),
    }
}
