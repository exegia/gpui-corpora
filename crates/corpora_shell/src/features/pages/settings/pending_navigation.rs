use gpui::App;

use crate::events::{AppEventKind, AppEventQueue};
use crate::platform::ipc::command::ForwardedCommand;

pub(super) fn take_settings_requests(cx: &mut App) -> Vec<String> {
    let requests: Vec<_> = cx
        .try_global::<AppEventQueue>()
        .into_iter()
        .flat_map(|queue| &queue.0)
        .filter_map(|event| {
            let path = match &event.kind {
                AppEventKind::Navigate(path) => path.clone(),
                AppEventKind::DeepLinkReceived(link)
                | AppEventKind::RemoteCommand(ForwardedCommand::DeepLink(link)) => {
                    crate::navigation::parse_deep_link(link).ok()?
                }
                _ => return None,
            };
            (path.split('/').nth(1) == Some("settings")).then_some((event.id, path))
        })
        .collect();
    if !requests.is_empty() {
        cx.global_mut::<AppEventQueue>()
            .0
            .retain(|event| !requests.iter().any(|(id, _)| *id == event.id));
    }
    requests.into_iter().map(|(_, path)| path).collect()
}
