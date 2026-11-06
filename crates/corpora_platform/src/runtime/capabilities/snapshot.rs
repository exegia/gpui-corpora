#[allow(unused_imports)]
use super::*;

pub fn snapshot(cx: &App) -> BTreeMap<String, CapabilityStatus> {
    cx.try_global::<CapabilityRegistry>()
        .map(|registry| registry.entries.clone())
        .unwrap_or_default()
}
