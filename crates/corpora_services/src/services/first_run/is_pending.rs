#[allow(unused_imports)]
use super::*;

pub fn is_pending(cx: &App) -> bool {
    !crate::app_state::with_config(cx, |c| c.first_run_completed)
}
