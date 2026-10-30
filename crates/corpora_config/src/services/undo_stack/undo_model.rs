#[allow(unused_imports)]
use super::*;

#[derive(Clone, Debug, Default)]
pub(crate) struct UndoModel {
    pub(crate) past: Vec<UndoEntry>,
    pub(crate) future: Vec<UndoEntry>,
    pub(crate) applying: bool,
    pub(crate) last_rejected: Option<String>,
}

impl UndoModel {
    pub(crate) fn from_state(state: UndoState) -> Self {
        Self {
            past: state.past,
            future: state.future,
            applying: state.applying,
            last_rejected: state.last_rejected,
        }
    }

    pub(crate) fn into_state(self) -> UndoState {
        UndoState {
            past: self.past,
            future: self.future,
            applying: self.applying,
            last_rejected: self.last_rejected,
        }
    }

    pub(crate) fn record(&mut self, entry: UndoEntry) {
        if self.applying {
            return;
        }
        self.past.push(entry);
        self.future.clear();
        self.last_rejected = None;
    }

    pub(crate) fn pop_undo(&mut self) -> Option<UndoEntry> {
        let entry = self.past.pop();
        if entry.is_none() {
            self.last_rejected = Some("nothing to undo".to_string());
        }
        entry
    }

    pub(crate) fn pop_redo(&mut self) -> Option<UndoEntry> {
        let entry = self.future.pop();
        if entry.is_none() {
            self.last_rejected = Some("nothing to redo".to_string());
        }
        entry
    }

    pub(crate) fn push_redo(&mut self, entry: UndoEntry) {
        self.future.push(entry);
        self.last_rejected = None;
    }

    pub(crate) fn push_undo(&mut self, entry: UndoEntry) {
        self.past.push(entry);
        self.last_rejected = None;
    }
}
