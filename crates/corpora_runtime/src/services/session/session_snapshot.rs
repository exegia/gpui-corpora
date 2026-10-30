#[allow(unused_imports)]
use super::*;

#[derive(Clone, Debug, Default)]
pub struct SessionSnapshot {
    pub state: SessionState,
}

impl Global for SessionSnapshot {}
