#[allow(unused_imports)]
use super::*;

#[derive(Clone, Debug, Default)]
pub struct CapabilityRegistry {
    pub(crate) entries: BTreeMap<String, CapabilityStatus>,
}

impl Global for CapabilityRegistry {}
