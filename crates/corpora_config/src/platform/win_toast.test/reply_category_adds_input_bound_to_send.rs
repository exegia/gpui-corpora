#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn reply_category_adds_input_bound_to_send() {
    let xml = actions_xml(&NotificationRequest::reply("Title", "Body"));
    assert_eq!(
        xml,
        format!(
            r#"<actions><input id="reply" type="text" placeHolderContent="{}"/><action content="{}" arguments="settings.reply" hint-inputId="reply"/></actions>"#,
            crate::i18n::localize("toast_reply_placeholder"),
            crate::i18n::localize("toast_send"),
        ),
        "the Send button must submit the reply input box"
    );
}
