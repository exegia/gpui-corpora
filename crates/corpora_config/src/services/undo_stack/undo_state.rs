#[allow(unused_imports)]
use super::*;

#[derive(Clone, Debug, Default)]
pub struct UndoState {
    pub past: Vec<UndoEntry>,
    pub future: Vec<UndoEntry>,
    pub applying: bool,
    pub last_rejected: Option<String>,
}

impl Global for UndoState {}
