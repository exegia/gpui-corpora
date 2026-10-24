#[allow(unused_imports)]
use super::*;

pub fn emit(kind: AppEventKind, cx: &mut App) {
    let event = AppEvent::new(kind);
    tracing::debug!(
        target: "corpora::events",
        event_id = %event.id,
        kind = ?event.kind,
        "emitting app event"
    );
    cx.default_global::<AppEventQueue>().0.push(event);
}
