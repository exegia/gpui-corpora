#[allow(unused_imports)]
use super::*;

pub(crate) fn toast_xml(request: &NotificationRequest) -> String {
    let actions = actions_xml(request);
    let audio = if request.play_sound {
        r#"<audio src="ms-winsoundevent:Notification.Default" />"#
    } else {
        r#"<audio silent="true" />"#
    };
    format!(
        r#"<toast duration="short"><visual><binding template="ToastGeneric"><text id="1">{}</text><text id="3">{}</text></binding></visual>{actions}{audio}</toast>"#,
        escaped(&request.title),
        escaped(&request.body),
    )
}
