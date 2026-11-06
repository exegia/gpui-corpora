#[allow(unused_imports)]
use super::*;

#[derive(Clone)]
pub struct StorageRuntime {
    pub(crate) backend: Arc<dyn StorageBackend>,
}

impl Global for StorageRuntime {}
