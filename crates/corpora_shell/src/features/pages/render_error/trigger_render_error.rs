#[allow(unused_imports)]
use super::*;

/// Activate the error boundary with a custom message. Render panics are
/// process-fatal in GPUI, so tests exercise the boundary via this action.
#[derive(Action, Clone, PartialEq, Eq, serde::Deserialize)]
#[action(namespace = app, no_json)]
pub struct TriggerRenderError {
    pub message: String,
}
