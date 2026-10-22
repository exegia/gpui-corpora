#[allow(unused_imports)]
use super::*;

pub fn check_now(cx: &mut App) {
    let probe_url = snapshot(cx).probe_url;
    // Degrade instead of panicking: init order shouldn't take down the UI.
    let Some((rt, client)) = crate::services::tokio_runtime::runtime_and_client(cx) else {
        cx.update_global::<ConnectivitySnapshot, _>(|next, _cx| {
            next.state = ConnectivityState::Unknown;
            next.last_error = Some("tokio runtime not installed; probe skipped".to_string());
        });
        tracing::warn!(
            target: "corpora::connectivity",
            "tokio runtime global not set; connectivity probe skipped"
        );
        return;
    };

    cx.spawn(async move |cx| {
        let probe_handle = rt.spawn(async move {
            client
                .get(&probe_url)
                .timeout(std::time::Duration::from_secs(10))
                .send()
                .await
        });

        // NetworkInterface::show() performs blocking system calls; run it on a
        // tokio blocking thread so it never stalls the main GPUI thread.
        let interfaces_handle = rt.spawn_blocking(read_interfaces);

        let result = match probe_handle.await {
            Ok(r) => r.map_err(|e| e.to_string()),
            Err(e) => Err(format!("connectivity probe panicked: {e}")),
        };

        let interfaces = interfaces_handle.await.unwrap_or_else(|e| {
            tracing::warn!(
                target: "corpora::connectivity",
                error = %e,
                "failed to read network interfaces"
            );
            Vec::new()
        });

        cx.update(move |cx| {
            cx.update_global::<ConnectivitySnapshot, _>(|next, _cx| {
                next.interfaces = interfaces;
                match result {
                    Ok(response) if response.status().is_success() => {
                        next.state = ConnectivityState::Online;
                        next.last_error = None;
                    }
                    Ok(response) => {
                        next.state = ConnectivityState::CaptiveOrFiltered;
                        next.last_error = Some(format!("probe status {}", response.status()));
                    }
                    Err(err) => {
                        next.state = ConnectivityState::Offline;
                        next.last_error = Some(err.to_string());
                    }
                }
                tracing::info!(
                    target: "corpora::connectivity",
                    state = ?next.state,
                    probe_url = %next.probe_url,
                    last_error = ?next.last_error,
                    interfaces = ?next.interfaces,
                    "connectivity probe completed"
                );
            });
        });
    })
    .detach();
}
