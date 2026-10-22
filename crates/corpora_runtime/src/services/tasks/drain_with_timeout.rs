#[allow(unused_imports)]
use super::*;

pub fn drain_with_timeout(timeout: Duration, cx: &mut App) -> Task<()> {
    request_shutdown();

    cx.spawn(async move |cx| {
        let start = Instant::now();
        loop {
            let active = cx.update(|cx| active_count(cx));
            if active == 0 {
                tracing::info!(
                    target: "corpora::tasks",
                    "all background tasks drained cooperatively"
                );
                return;
            }
            if start.elapsed() >= timeout {
                tracing::warn!(
                    target: "corpora::tasks",
                    elapsed_ms = start.elapsed().as_millis() as u64,
                    remaining = active,
                    "drain deadline exceeded, force-cancelling remaining tasks"
                );
                cx.update(force_cancel_remaining);
                return;
            }
            cx.background_executor()
                .timer(Duration::from_millis(100))
                .await;
        }
    })
}
