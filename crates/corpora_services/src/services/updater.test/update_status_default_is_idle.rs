#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn update_status_default_is_idle() {
    assert_eq!(UpdateStatus::default(), UpdateStatus::Idle);
}
