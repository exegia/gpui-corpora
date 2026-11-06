#[allow(unused_imports)]
use super::*;

pub(crate) fn open_with_splash(cx: &mut App) {
    let splash = super::splash_artwork::open(cx).expect("splash failed to open");

    cx.spawn(async move |cx| {
        // A short presentation interval lets the splash paint before the workspace.
        cx.background_executor()
            .timer(Duration::from_millis(400))
            .await;
        cx.update(|cx| {
            let (_, workspace) =
                crate::app::open_workspace_window("Corpora", cx).expect("workspace failed to open");
            match tray(cx) {
                Ok(tray) => {
                    workspace.update(cx, |workspace, _| {
                        workspace.tray = Some(tray);
                    });
                }
                Err(error) => {
                    tracing::warn!(target: "corpora::startup", %error, "tray unavailable")
                }
            }
            if let Err(error) = set_dock_badge(None) {
                tracing::debug!(target: "corpora::startup", %error, "dock badge unavailable");
            }
            splash
                .update(cx, |_, window, _| window.remove_window())
                .ok();
            cx.activate(true);
        });
    })
    .detach();
}
