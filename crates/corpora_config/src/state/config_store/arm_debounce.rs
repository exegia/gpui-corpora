#[allow(unused_imports)]
use super::*;

/// Arm the debounced flush timer unless one is already pending; the
/// `SAVE_SCHEDULED` compare_exchange keeps at most one timer in flight.
pub(crate) fn arm_debounce(cx: &mut App) {
    if SAVE_SCHEDULED
        .compare_exchange(false, true, Ordering::Relaxed, Ordering::Relaxed)
        .is_err()
    {
        return;
    }

    let bg = cx.background_executor().clone();
    let task = cx.spawn(async move |cx| {
        bg.timer(std::time::Duration::from_millis(DEBOUNCE_MS))
            .await;

        SAVE_SCHEDULED.store(false, Ordering::Relaxed);

        // No I/O here.
        let request = cx.update(|cx| {
            cx.update_global::<AppState, _>(|state, _cx| {
                if !state.dirty {
                    return None;
                }
                prepare_flush(state)
            })
        });

        let Some((path, bytes)) = request else {
            return;
        };

        let write_bytes = bytes.clone();
        let result = bg
            .spawn(async move { save_config(&path, &write_bytes) })
            .await;

        cx.update(|cx| {
            let needs_rearm =
                cx.update_global::<AppState, _>(|state, _cx| commit_flush(state, bytes, result));
            if needs_rearm {
                arm_debounce(cx);
            }
        });
    });

    cx.update_global::<AppState, _>(|state, _cx| {
        state.in_flight_save = Some(task);
    });
}
