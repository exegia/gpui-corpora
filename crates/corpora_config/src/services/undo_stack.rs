use crate::app::theme::ThemeMode;
use gpui::{App, Global};

use crate::time::AppTimestamp;

#[cfg(test)]
#[path = "undo_stack.test/mod.rs"]
mod undo_stack_test;

mod apply_forward;
mod apply_inverse;
mod can_redo;
mod can_undo;
mod initialize;
mod record_theme_mode_change;
mod redo;
mod snapshot;
mod undo;
mod undo_entry;
mod undo_kind;
mod undo_model;
mod undo_state;

use apply_forward::apply_forward;
use apply_inverse::apply_inverse;
pub use can_redo::can_redo;
pub use can_undo::can_undo;
pub use initialize::initialize;
pub use record_theme_mode_change::record_theme_mode_change;
pub use redo::redo;
pub use snapshot::snapshot;
pub use undo::undo;
pub use undo_entry::UndoEntry;
pub use undo_kind::UndoKind;
use undo_model::UndoModel;
pub use undo_state::UndoState;
