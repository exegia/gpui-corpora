#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn pop_redo_sets_rejected_reason_when_empty() {
    let mut model = UndoModel::default();
    assert!(model.pop_redo().is_none());
    assert_eq!(model.last_rejected.as_deref(), Some("nothing to redo"));
}
