#[allow(unused_imports)]
use super::*;

pub fn initialize(cx: &mut App) {
    cx.set_global(CapabilityRegistry::default());
}
