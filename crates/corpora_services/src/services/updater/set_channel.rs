#[allow(unused_imports)]
use super::*;

pub fn set_channel(channel: &str, cx: &mut gpui::App) {
    let ch = if channel.is_empty() {
        "stable".to_string()
    } else {
        channel.to_string()
    };
    UpdateSnapshot::update_global(cx, |snap, _cx| {
        snap.update_channel = ch.clone();
    });
    crate::app_state::update_config(cx, |config| {
        config.update_channel = ch;
    });
    tracing::info!(
        target: "corpora::updater",
        channel = %channel,
        "update channel set"
    );
}
