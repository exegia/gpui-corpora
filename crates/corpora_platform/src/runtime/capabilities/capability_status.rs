#[allow(unused_imports)]
use super::*;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapabilityStatus {
    pub supported: bool,
    pub enabled: bool,
    pub degraded: bool,
    pub reason: Option<SharedString>,
    pub last_error: Option<SharedString>,
}

impl CapabilityStatus {
    pub fn supported_enabled() -> Self {
        Self {
            supported: true,
            enabled: true,
            degraded: false,
            reason: None,
            last_error: None,
        }
    }

    /// Supported but a sub-component failed: stays enabled, flagged degraded,
    /// with `err` as both reason and last_error.
    pub fn degraded(err: impl ToString) -> Self {
        let msg: SharedString = err.to_string().into();
        Self {
            supported: true,
            enabled: true,
            degraded: true,
            reason: Some(msg.clone()),
            last_error: Some(msg),
        }
    }

    /// Failed to initialize entirely: supported but not enabled, flagged
    /// degraded, with `err` as both reason and last_error.
    pub fn error(err: impl ToString) -> Self {
        let msg: SharedString = err.to_string().into();
        Self {
            supported: true,
            enabled: false,
            degraded: true,
            reason: Some(msg.clone()),
            last_error: Some(msg),
        }
    }
}
