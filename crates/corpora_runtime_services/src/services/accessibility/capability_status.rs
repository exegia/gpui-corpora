#[allow(unused_imports)]
use super::*;

/// Capability entry for the registry: the a11y infrastructure is always
/// enabled; per-window activation stays dynamic and is tracked in the
/// snapshot instead.
pub fn capability_status() -> CapabilityStatus {
    CapabilityStatus {
        supported: true,
        enabled: true,
        degraded: false,
        reason: Some(
            "accesskit bridge activates per window when assistive technology connects".into(),
        ),
        last_error: None,
    }
}
