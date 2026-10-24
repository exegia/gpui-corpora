#[allow(unused_imports)]
use super::*;

pub fn initialize(cx: &mut gpui::App) {
    let channel = crate::app_state::update_channel(cx);
    cx.set_global(UpdateSnapshot {
        status: UpdateStatus::Idle,
        current_version: types::current_app_version(),
        last_check: None,
        update_channel: if channel.is_empty() {
            "stable".to_string()
        } else {
            channel
        },
        check_retry_count: 0,
        download_retry_count: 0,
        ..Default::default()
    });

    cx.on_action(|_: &CheckForUpdates, cx| {
        check::check_for_updates(cx);
    });

    // GPUI background-executor timers instead of a spawned tokio sleep task.
    cx.spawn(async move |cx| {
        cx.background_executor()
            .timer(std::time::Duration::from_secs(
                types::STARTUP_CHECK_DELAY_SECS,
            ))
            .await;
        cx.update(|cx| {
            tracing::info!(
                target: "corpora::updater",
                "running startup update check"
            );
            check::check_for_updates(cx);
        });
    })
    .detach();

    cx.spawn(async move |cx| {
        loop {
            cx.background_executor()
                .timer(std::time::Duration::from_secs(
                    types::PERIODIC_CHECK_INTERVAL_SECS,
                ))
                .await;

            let should_check: bool = cx.update(|cx| {
                let snap = snapshot(cx);
                matches!(
                    snap.status,
                    UpdateStatus::Idle | UpdateStatus::UpToDate | UpdateStatus::Error(_)
                )
            });
            if should_check {
                cx.update(|cx| {
                    tracing::info!(
                        target: "corpora::updater",
                        "running periodic update check"
                    );
                    check::check_for_updates(cx);
                });
            }
        }
    })
    .detach();

    tracing::info!(
        target: "corpora::updater",
        version = %env!("CARGO_PKG_VERSION"),
        "updater service initialized"
    );
}
