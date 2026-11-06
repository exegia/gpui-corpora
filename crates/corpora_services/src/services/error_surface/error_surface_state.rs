#[allow(unused_imports)]
use super::*;

/// In-memory error surface state (primary store).
#[derive(Clone, Debug, Default)]
pub struct ErrorSurfaceState {
    pub records: Vec<ErrorRecord>,
}

impl Global for ErrorSurfaceState {}
