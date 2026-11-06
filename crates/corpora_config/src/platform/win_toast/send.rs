#[allow(unused_imports)]
use super::*;

pub fn send(request: &NotificationRequest, sink: &ResponseSink) -> anyhow::Result<()> {
    let tag = toast_tag(request);

    let xml = XmlDocument::new()?;
    xml.LoadXml(&HSTRING::from(toast_xml(request)))?;

    let toast = ToastNotification::CreateToastNotification(&xml)?;
    toast.SetTag(&HSTRING::from(&tag))?;
    toast.SetGroup(&HSTRING::from(MESSAGE_GROUP))?;

    // WinRT interface types are !Send, so every WinRT object stays local to
    // this frame; the closures below capture only Send types (sink + tag).
    let activated = {
        let sink = Arc::clone(sink);
        let tag = tag.clone();
        TypedEventHandler::new(move |_, args: Ref<'_, IInspectable>| {
            let response = ToastResponse {
                tag: tag.clone(),
                action: activated_action(&args),
                user_text: reply_text(&args),
            };
            if let Some(handler) = sink.get() {
                handler(response);
            }
            Ok(())
        })
    };
    let dismissed = {
        let sink = Arc::clone(sink);
        let tag = tag.clone();
        TypedEventHandler::new(move |_, args: Ref<'_, ToastDismissedEventArgs>| {
            let user_canceled = args
                .as_ref()
                .and_then(|args| args.Reason().ok())
                .is_some_and(|reason| reason == ToastDismissalReason::UserCanceled);
            if user_canceled && let Some(handler) = sink.get() {
                handler(ToastResponse {
                    tag: tag.clone(),
                    action: ResponseAction::Dismiss,
                    user_text: None,
                });
            }
            Ok(())
        })
    };

    toast.Activated(&activated)?;
    toast.Dismissed(&dismissed)?;

    let notifier =
        ToastNotificationManager::CreateToastNotifierWithId(&HSTRING::from(DESKTOP_ENTRY_ID))?;
    notifier.Show(&toast)?;
    tracing::debug!(target: LOG, tag = %tag, "interactive toast shown");
    Ok(())
}
