use corpora_ui::ely::theme::Theme;
use gpui_kit::{App, KeyBinding};

use crate::app::actions::*;
use crate::app::locale::set_locale;
use crate::app::theme::set_theme_mode;

/// Time and log a named startup step; the log name always matches the step.
macro_rules! startup_step {
    ($cx:expr, $name:expr, $body:block) => {{
        crate::lifecycle::set_startup_step($name, $cx);
        let _t = std::time::Instant::now();
        $body;
        tracing::info!(
            target: "corpora::startup",
            elapsed_ms = _t.elapsed().as_millis() as u64,
            "{} done",
            $name
        );
    }};
}

/// App services and globals. Runs after `corpora_ui::ely::init`, before
/// any window opens.
pub fn init(cx: &mut App) {
    let startup_start = std::time::Instant::now();

    crate::lifecycle::install_panic_hook();

    crate::lifecycle::set_stage(crate::lifecycle::LifecycleStage::Starting, cx);
    startup_step!(cx, "app_state_init", {
        crate::app_state::initialize(cx);
    });

    // Marker data must be user-owned, never $TMPDIR (symlink planting);
    // install it before the marker write so startup crashes stay detectable.
    crate::lifecycle::set_app_data_dir(crate::app_state::paths(cx).data_dir.clone());
    let previous_crash = crate::lifecycle::check_previous_crash();
    if let Some(marker) = &previous_crash {
        tracing::warn!(
            target: "corpora::lifecycle",
            marker = %marker,
            "previous crash detected"
        );
    }
    crate::lifecycle::write_crash_marker();

    startup_step!(cx, "logging_init", {
        crate::logging::initialize(cx);
    });

    // Best-effort per-user corpora:// registration; never block startup.
    #[cfg(target_os = "windows")]
    startup_step!(cx, "deep_link_registration", {
        if let Err(reason) = crate::platform::deep_link_registration::ensure_deep_link_registered()
        {
            tracing::warn!(
                target: "corpora::deep_link_registration",
                %reason,
                "deep-link scheme registration failed; continuing without it"
            );
        }
    });

    startup_step!(cx, "capabilities_init", {
        crate::capabilities::initialize(cx);
    });

    const DEFAULT_ENABLED_CAPABILITIES: &[&str] = &[
        "app_state",
        "deep_links",
        "diagnostics",
        "notification_inbox",
        "background_tasks",
        "status_bar",
        "connectivity",
        "session",
        "first_run",
        "launcher",
        "app_menu",
        "command_registry",
    ];
    for &name in DEFAULT_ENABLED_CAPABILITIES {
        crate::capabilities::set(
            name,
            crate::capabilities::CapabilityStatus::supported_enabled(),
            cx,
        );
    }

    let system_locale = crate::i18n::detect_system_locale();
    tracing::info!(
        target: "corpora::startup",
        system_locale = %system_locale,
        "detected system locale"
    );
    crate::app::locale::install_ely_i18n(cx);

    let persisted = crate::app_state::config(cx);
    let locale_to_use = if persisted.locale.is_empty() {
        system_locale
    } else {
        persisted.locale.clone()
    };
    set_locale(&locale_to_use, cx);

    // Install before set_mode_now: it cancels set_palette's fade and jumps to the new palette.
    corpora_ui::styles::tokens::MacosClassic::new().install(cx);
    let appearance = cx.window_appearance();
    Theme::set_mode_now(persisted.theme.resolve(appearance), cx);
    gpui_kit::component::Theme::change(persisted.theme.kit_mode(appearance), None, cx);

    cx.on_action(|switch: &SwitchThemeMode, cx| {
        set_theme_mode(switch.0, cx);
    });
    cx.on_action(|locale: &SelectLocale, cx| {
        set_locale(&locale.0, cx);
    });

    cx.set_global(crate::events::AppEventQueue::default());
    startup_step!(cx, "runtime_services_init", {
        crate::tasks::initialize(cx);
        crate::error_surface::initialize(cx);
        crate::undo_stack::initialize(cx);
        crate::shortcuts::initialize(cx);
        cx.set_global(crate::services::tokio_runtime::TokioRuntimeGlobal(
            crate::services::tokio_runtime::TokioRuntime::new(),
        ));
        crate::connectivity::initialize(cx);
        // Probe once at startup so the status bar and the update checker see
        // a real state instead of Unknown (the updater refuses while Unknown).
        crate::connectivity::check_now(cx);
        crate::desktop_actions::initialize(cx);
        crate::accessibility::initialize(cx);
        crate::secure_storage::initialize(cx);
        crate::session::initialize(cx);
        crate::storage::initialize(cx);

        crate::lifecycle::set_startup_step("db_migrations", cx);
        let migrations_t = std::time::Instant::now();
        if let Some(snapshot) = cx.try_global::<crate::storage::StorageSnapshot>()
            && snapshot.available
        {
            let db_path = std::path::PathBuf::from(snapshot.db_path.clone());
            match rusqlite::Connection::open(&db_path) {
                Ok(conn) => match crate::db_migrations::run_migrations(&conn) {
                    Ok(version) => {
                        tracing::info!(
                            target: "corpora::startup",
                            version,
                            elapsed_ms = migrations_t.elapsed().as_millis() as u64,
                            "db_migrations complete"
                        );
                    }
                    Err(err) => {
                        tracing::error!(
                            target: "corpora::startup",
                            error = %err,
                            "db_migrations failed"
                        );
                        crate::lifecycle::set_startup_error(format!("migration failed: {err}"), cx);
                    }
                },
                Err(err) => {
                    tracing::error!(
                        target: "corpora::startup",
                        error = %err,
                        "failed to open db for migrations"
                    );
                }
            }
        }

        crate::telemetry::initialize(cx);
    });
    // Window counts in the a11y snapshot go stale on close; refresh keeps the
    // diagnostics page honest.
    cx.on_window_closed(|cx, _| crate::accessibility::refresh(cx))
        .detach();
    crate::crash_report::initialize(cx);
    if previous_crash.is_some() {
        crate::crash_report::upload_pending_reports(cx);
    }
    crate::telemetry::record_event("app_runtime_initialized", cx);
    crate::notifications::inbox::initialize(cx);
    crate::notifications::initialize(cx);
    crate::notifications::set_native_notifications_enabled(
        persisted.native_notifications_enabled,
        cx,
    );
    crate::services::updater::initialize(cx);
    crate::services::updater::check_pending_swap(cx);

    cx.bind_keys([
        KeyBinding::new(crate::app::keys::TOGGLE_SEARCH, ToggleSearch, None),
        KeyBinding::new("/", ToggleSearch, None),
        #[cfg(target_os = "macos")]
        KeyBinding::new("secondary-q", Quit, None),
        #[cfg(not(target_os = "macos"))]
        KeyBinding::new("alt-f4", Quit, None),
        #[cfg(any(unix, windows))]
        KeyBinding::new("ctrl-r", Restart, None),
    ]);

    cx.on_action(|_: &Quit, cx| {
        crate::lifecycle::set_shutdown_step("begin_shutdown", cx);
        crate::lifecycle::set_stage(crate::lifecycle::LifecycleStage::ShuttingDown, cx);

        // Persist the final window position even if the debounce has not fired.
        crate::lifecycle::set_shutdown_step("flush_window_bounds", cx);
        crate::app::window::flush_window_bounds(cx);

        crate::lifecycle::set_shutdown_step("drain_tasks", cx);

        let drain = crate::tasks::drain_with_timeout(std::time::Duration::from_secs(5), cx);

        cx.spawn(async move |cx| {
            drain.await;

            cx.update(|cx| {
                crate::lifecycle::set_shutdown_step("stop_ipc", cx);
                crate::single_instance::shutdown(cx);
                crate::lifecycle::set_shutdown_step("stop_watchers", cx);
                crate::desktop_actions::shutdown(cx);
                crate::lifecycle::set_shutdown_step("unregister_shortcuts", cx);
                crate::shortcuts::shutdown(cx);
                crate::lifecycle::set_shutdown_step("flush_config", cx);
                crate::app_state::force_save(cx);
                crate::lifecycle::set_shutdown_step("flush_storage", cx);
                crate::storage::shutdown(cx);
                crate::lifecycle::set_shutdown_step("flush_telemetry", cx);
                crate::telemetry::record_event("app_shutdown_requested", cx);
                crate::telemetry::shutdown(cx);
                crate::lifecycle::set_shutdown_step("flush_logs", cx);
                crate::logging::shutdown(cx);
                crate::lifecycle::set_shutdown_step("flush_crash_reports", cx);
                crate::crash_report::shutdown(cx);
                crate::lifecycle::set_shutdown_step("remove_crash_marker", cx);
                crate::lifecycle::remove_crash_marker();
                crate::lifecycle::set_shutdown_step("quit", cx);
                cx.quit();
            });
        })
        .detach();
    });

    cx.on_action(|_: &Restart, cx| {
        // Flag the relaunch, then reuse the full Quit shutdown path so every
        // flush runs before the process exits.
        #[cfg(any(unix, windows))]
        {
            crate::app::request_reload();
            crate::lifecycle::set_shutdown_step("restart", cx);
        }
        #[cfg(not(any(unix, windows)))]
        {
            let _ = cx;
            tracing::warn!(
                target: "corpora::reload",
                "restart requested on a platform without a native relaunch backend; ignoring"
            );
        }
        #[cfg(any(unix, windows))]
        {
            // The handler window is mid-teardown or on the update stack, so
            // defer Quit, routed at the recorded root (active is the palette).
            cx.defer(crate::app::window::dispatch_quit);
        }
    });

    cx.on_action(|_: &About, cx| {
        crate::events::emit(
            crate::events::AppEventKind::Navigate(crate::navigation::ABOUT.to_string()),
            cx,
        );
    });
    cx.on_action(|_: &OpenDiagnostics, cx| {
        crate::events::emit(
            crate::events::AppEventKind::Navigate(crate::navigation::DIAGNOSTICS.to_string()),
            cx,
        );
    });
    #[cfg(debug_assertions)]
    cx.on_action(|_: &TriggerTestPanic, _cx| {
        panic!("corpora test panic action");
    });
    cx.on_action(|command: &ExecuteCommand, cx| {
        let availability = crate::commands::availability(command.0, cx);
        if !availability.enabled {
            let reason = availability
                .disabled_reason
                .as_ref()
                .map(|value| value.to_string())
                .unwrap_or_else(|| "command disabled".to_string());
            tracing::warn!(
                target: "corpora::commands",
                command = ?command.0,
                reason = %reason,
                "command ignored"
            );
            return;
        }
        crate::commands::execute(command.0, cx);
    });

    cx.activate(true);
    crate::lifecycle::set_startup_step("running", cx);
    crate::lifecycle::set_stage(crate::lifecycle::LifecycleStage::Running, cx);

    tracing::info!(
        target: "corpora::startup",
        total_elapsed_ms = startup_start.elapsed().as_millis() as u64,
        "startup complete"
    );
}
