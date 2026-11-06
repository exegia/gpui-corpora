#[allow(unused_imports)]
use super::*;

/// Request envelope; `id` correlates the [`ForwardedResponse`].
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ForwardedRequest {
    pub id: u64,
    pub command: ForwardedCommand,
}

impl ForwardedRequest {
    pub fn new(id: u64, command: ForwardedCommand) -> Self {
        Self { id, command }
    }
}
