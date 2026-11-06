#[allow(unused_imports)]
use super::*;

pub fn set(name: impl Into<String>, status: CapabilityStatus, cx: &mut App) {
    let key = name.into();
    tracing::debug!(
        target: "corpora::capabilities",
        capability = %key,
        supported = status.supported,
        enabled = status.enabled,
        degraded = status.degraded,
        reason = ?status.reason,
        last_error = ?status.last_error,
        "capability updated"
    );
    cx.default_global::<CapabilityRegistry>()
        .entries
        .insert(key, status);
}
