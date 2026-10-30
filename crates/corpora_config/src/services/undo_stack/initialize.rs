#[allow(unused_imports)]
use super::*;

pub fn initialize(cx: &mut App) {
    cx.set_global(UndoState::default());
    crate::capabilities::set(
        "undo_stack",
        crate::capabilities::CapabilityStatus::supported_enabled(),
        cx,
    );
}
