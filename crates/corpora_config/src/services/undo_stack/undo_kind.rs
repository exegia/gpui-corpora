#[allow(unused_imports)]
use super::*;

#[derive(Clone, Debug)]
pub enum UndoKind {
    ThemeMode { before: ThemeMode, after: ThemeMode },
}
