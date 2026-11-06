#[allow(unused_imports)]
use super::*;

pub fn initialize(cx: &mut App) {
    let paths = crate::app_state::paths(cx);
    let log_dir = paths.log_dir.display().to_string();
    let file_prefix = "corpora.log".to_string();

    let file_appender = tracing_appender::rolling::daily(&paths.log_dir, &file_prefix);
    let (file_writer, guard) = tracing_appender::non_blocking(file_appender);

    let init_result = tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(
            tracing_subscriber::fmt::layer()
                .with_ansi(false)
                .with_writer(file_writer),
        )
        // gpui's `.log_err()` strips its target to a `crates/` path segment the
        // registry path lacks, emitting empty-target noise (e.g. a splash-window
        // teardown frame logging "window not found"). Drop that category.
        .with(tracing_subscriber::filter::filter_fn(|meta| {
            !meta.target().is_empty()
        }))
        .with(env_filter())
        .try_init();

    initialize_state(cx, log_dir, file_prefix, init_result, Some(guard));
}
