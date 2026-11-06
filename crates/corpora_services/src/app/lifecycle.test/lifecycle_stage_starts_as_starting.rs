#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn lifecycle_stage_starts_as_starting() {
    let state = LifecycleState::default();
    assert_eq!(state.stage, LifecycleStage::Starting);
}
