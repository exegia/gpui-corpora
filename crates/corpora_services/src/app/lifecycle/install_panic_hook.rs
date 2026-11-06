#[allow(unused_imports)]
use super::*;

pub fn install_panic_hook() {
    let previous = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        let summary = info.to_string();
        let slot = LAST_PANIC_SUMMARY.get_or_init(|| Mutex::new(None));
        if let Ok(mut value) = slot.lock() {
            *value = Some(summary.clone());
        }

        let is_render = in_render_path();

        // Only render-originating panics swap in the error boundary; other
        // panics (background tasks, init) must not.
        if is_render {
            RENDER_PANIC_OCCURRED.store(true, Ordering::SeqCst);
        }

        if let Some(data_dir) = APP_DATA_DIR.get() {
            let backtrace = std::backtrace::Backtrace::capture();
            let bt_string = match backtrace.status() {
                std::backtrace::BacktraceStatus::Captured => backtrace.to_string(),
                _ => String::new(),
            };

            let recent_errors = RECENT_ERRORS
                .get()
                .and_then(|slot| slot.lock().ok())
                .map(|guard| guard.clone())
                .unwrap_or_default();

            let report = crate::services::crash_report::CrashReport::new(
                summary.clone(),
                bt_string,
                is_render,
                recent_errors,
            );

            if let Err(err) = crate::services::crash_report::write_crash_report(&report, data_dir) {
                // We are inside the panic handler -- best-effort logging only.
                eprintln!("[corpora::lifecycle] failed to write crash report: {err}");
            }
        }

        tracing::error!(
            target: "corpora::lifecycle",
            panic = %summary,
            "application panic captured"
        );
        previous(info);
    }));
}
