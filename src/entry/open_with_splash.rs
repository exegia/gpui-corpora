#[allow(unused_imports)]
use super::*;

pub(crate) fn open_with_splash(cx: &mut App) {
    let splash = SplashScreen::new("Corpora")
        .icon(IconName::BookOpen)
        .tagline("Your corpus workspace")
        .status("Opening workspace…")
        .open(cx)
        .expect("splash failed to open");

    cx.spawn(async move |cx| {
        // A short presentation interval lets the splash paint before the workspace.
        cx.background_executor()
            .timer(Duration::from_millis(400))
            .await;
        cx.update(|cx| {
            let handle = crate::app::open_workspace_window("Corpora", cx)
                .expect("workspace failed to open");
            match tray(cx) {
                Ok(tray) => {
                    if let Err(error) = handle.update(cx, |workspace, _, _| {
                        workspace.tray = Some(tray);
                    }) {
                        tracing::error!(target: "corpora::startup", %error, "workspace closed during startup");
                    }
                }
                Err(error) => {
                    tracing::warn!(target: "corpora::startup", %error, "tray unavailable")
                }
            }
            if let Err(error) = set_dock_badge(None) {
                tracing::debug!(target: "corpora::startup", %error, "dock badge unavailable");
            }
            splash.update(cx, |_, window, _| window.remove_window()).ok();
            cx.activate(true);
        });
    })
    .detach();
}
