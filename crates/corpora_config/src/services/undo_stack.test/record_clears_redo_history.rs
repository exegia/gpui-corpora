#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn record_clears_redo_history() {
    let mut model = UndoModel {
        future: vec![sample_entry()],
        ..UndoModel::default()
    };
    model.record(sample_entry());
    assert_eq!(model.past.len(), 1);
    assert!(model.future.is_empty());
}
