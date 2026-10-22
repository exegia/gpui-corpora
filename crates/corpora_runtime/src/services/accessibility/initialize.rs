#[allow(unused_imports)]
use super::*;

pub fn initialize(cx: &mut App) {
    refresh(cx);
    let stored = snapshot(cx);
    crate::capabilities::set("accessibility", capability_status(), cx);
    tracing::info!(
        target: "corpora::accessibility",
        status = %stored.status,
        "accessibility initialized"
    );
}
