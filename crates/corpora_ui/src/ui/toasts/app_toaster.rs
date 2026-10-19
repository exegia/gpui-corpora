#[allow(unused_imports)]
use super::*;

pub(crate) struct AppToaster {
    pub(crate) toaster: Entity<Toaster>,
    /// Keyed toasts replace their previous instance instead of stacking.
    pub(crate) keyed: HashMap<String, u64>,
}

impl Global for AppToaster {}
