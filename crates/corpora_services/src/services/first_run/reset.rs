#[allow(unused_imports)]
use super::*;

pub fn reset(cx: &mut App) {
    crate::app_state::update_config(cx, |config| {
        config.first_run_completed = false;
    });
    tracing::info!(target: "corpora::first_run", "first-run reset");
}
