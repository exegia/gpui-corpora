#[allow(unused_imports)]
use super::*;

pub fn set_state(state: SessionState, cx: &mut App) {
    tracing::info!(target: "corpora::session", state = ?state, "session state updated");
    cx.update_global::<SessionSnapshot, _>(|snapshot, _cx| {
        snapshot.state = state;
    });
}
