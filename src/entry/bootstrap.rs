#[allow(unused_imports)]
use super::*;

/// Boot the application; blocks until the app quits.
pub fn bootstrap() {
    let preflight = crate::single_instance::preflight();
    if !preflight.should_start {
        return;
    }
    let startup_runtime = preflight.runtime;
    let startup_deep_link = preflight.initial_deep_link;

    // `run` blocks for the whole app lifetime, so the stack frame keeps the App alive.
    gpui_platform::application()
        .with_assets(Assets)
        .run(move |cx: &mut App| {
            ely_gpui_component::init(cx).expect("Ely failed to start");
            gpui_router::init(cx);
            crate::app::init(cx);
            crate::app::menus::init("Corpora", cx);
            if let Some(runtime) = startup_runtime {
                crate::single_instance::install(runtime, cx);
            }
            // Queued until the workspace drains the event queue on open.
            if let Some(link) = startup_deep_link {
                crate::events::emit(crate::events::AppEventKind::DeepLinkReceived(link), cx);
            }
            open_with_splash(cx);

            cx.on_window_closed(|cx, _| {
                if cx.windows().is_empty() {
                    let _ = set_dock_badge(None);
                    crate::app::window::dispatch_quit(cx);
                }
            })
            .detach();
        });

    // Runs post-run only: the instance-mutex Global drops inside run()'s
    // frame, and spawning outside any App borrow avoids gpui's Win32
    // message-pump re-entrancy hazard. Never spawn from the Restart handler.
    if crate::app::is_reload_requested()
        && let Err(err) = crate::app::perform_reload()
    {
        eprintln!("reload failed: {err}");
    }
}
