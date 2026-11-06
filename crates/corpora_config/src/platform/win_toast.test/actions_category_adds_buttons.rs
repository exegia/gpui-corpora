#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn actions_category_adds_buttons() {
    let xml = toast_xml(&NotificationRequest::action_buttons("Title", "Body"));
    assert_eq!(
        actions_xml(&NotificationRequest::action_buttons("Title", "Body")),
        format!(
            r#"<actions><action content="{}" arguments="settings.open" activationType="foreground"/><action content="{}" arguments="settings.snooze" activationType="foreground"/></actions>"#,
            crate::i18n::localize("toast_open"),
            crate::i18n::localize("toast_snooze"),
        ),
        "button ids must match the macOS category registrations"
    );
    assert!(xml.contains(r#"<text id="1">Title</text>"#));
    assert!(xml.contains(r#"<text id="3">Body</text>"#));
}
