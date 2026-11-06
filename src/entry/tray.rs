#[allow(unused_imports)]
use super::*;

pub(crate) fn tray(cx: &mut App) -> anyhow::Result<TrayIcon> {
    let tray = TrayIcon::new(
        "books.vertical",
        vec![
            TrayItem::Action("Show Corpora".into()),
            TrayItem::Action("Home".into()),
            TrayItem::Action("Explore".into()),
            TrayItem::Action("Projects".into()),
            TrayItem::Separator,
            TrayItem::Action("Quit Corpora".into()),
        ],
        move |index, cx| {
            if index == TRAY_QUIT {
                let _ = set_dock_badge(None);
                crate::app::window::dispatch_quit(cx);
                return;
            }
            cx.activate(true);
            if let Some(window) = crate::app::window::root_window(cx) {
                window
                    .update(cx, |_, window, _| window.activate_window())
                    .ok();
            }
            if let Some(Some(path)) = TRAY_PATHS.get(index) {
                crate::events::emit(crate::events::AppEventKind::Navigate(path.to_string()), cx);
            }
        },
        cx,
    )?;
    // Ely only accepts an SF Symbol; swap in the Corpora mark. The symbol
    // stays as the fallback if AppKit can't give us the button.
    if let Err(error) = corpora_shell::platform::apply_tray_icon() {
        tracing::warn!(target: "corpora::startup", %error, "tray keeps its symbol icon");
    }
    Ok(tray)
}
