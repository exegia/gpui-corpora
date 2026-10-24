#[allow(unused_imports)]
use super::*;

#[derive(Clone)]
pub struct DesktopActionsState {
    pub(crate) snapshot: DesktopActionsSnapshot,
    pub(crate) inner: Arc<Mutex<DesktopActionsInner>>,
}

impl Global for DesktopActionsState {}
