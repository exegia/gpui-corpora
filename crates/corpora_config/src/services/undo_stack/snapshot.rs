#[allow(unused_imports)]
use super::*;

pub fn snapshot(cx: &App) -> UndoState {
    cx.try_global::<UndoState>().cloned().unwrap_or_default()
}
