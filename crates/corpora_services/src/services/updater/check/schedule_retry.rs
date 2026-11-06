#[allow(unused_imports)]
use super::*;

/// Returns false when retries are exhausted — the caller owns the terminal
/// error path and the interim status.
pub(crate) fn schedule_retry<F, R>(
    cx: &mut App,
    retry_field: F,
    retry_kind: &'static str,
    rearm: R,
) -> bool
where
    F: Fn(&mut UpdateSnapshot) -> &mut u32,
    R: FnOnce(&mut App) + Send + 'static,
{
    let retry_count = *retry_field(cx.global_mut::<UpdateSnapshot>());
    if retry_count >= MAX_UPDATE_RETRIES {
        return false;
    }
    let new_count = retry_count + 1;
    let delay_secs = RETRY_BASE_DELAY_SECS * 2u64.pow(new_count - 1);
    tracing::warn!(
        target: "corpora::updater",
        kind = retry_kind,
        retry = new_count,
        max = MAX_UPDATE_RETRIES,
        delay_secs,
        "update operation failed — scheduling retry"
    );
    UpdateSnapshot::update_global(cx, |snap, _cx| {
        *retry_field(snap) = new_count;
    });

    cx.spawn(async move |cx| {
        cx.background_executor()
            .timer(std::time::Duration::from_secs(delay_secs))
            .await;
        cx.update(|cx| {
            tracing::info!(
                target: "corpora::updater",
                kind = retry_kind,
                retry = new_count,
                "retrying update operation"
            );
            rearm(cx);
        });
    })
    .detach();
    true
}
