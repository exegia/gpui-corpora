#[allow(unused_imports)]
use super::*;

/// Reply to a [`ForwardedRequest`], keyed by the same `id`. The error string
/// is best-effort and log-safe, not for verbatim display to end users.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ForwardedResponse {
    pub id: u64,
    pub ok: bool,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub error: Option<String>,
}

impl ForwardedResponse {
    pub fn ok(id: u64) -> Self {
        Self {
            id,
            ok: true,
            error: None,
        }
    }

    pub fn error(id: u64, error: impl Into<String>) -> Self {
        let message = error.into();
        tracing::warn!(target: LOG, id, error = %message, "ipc request failed");
        Self {
            id,
            ok: false,
            error: Some(message),
        }
    }
}
