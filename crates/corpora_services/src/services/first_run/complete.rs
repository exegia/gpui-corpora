#[allow(unused_imports)]
use super::*;

pub fn complete(cx: &mut App) {
    crate::app_state::update_config(cx, |config| {
        config.first_run_completed = true;
    });
    tracing::info!(target: "corpora::first_run", "first-run marked complete");
}
