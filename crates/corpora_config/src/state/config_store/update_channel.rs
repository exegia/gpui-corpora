#[allow(unused_imports)]
use super::*;

/// Convenience getter cloning just the update channel instead of the config.
pub fn update_channel(cx: &App) -> String {
    with_config(cx, |c| c.update_channel.clone())
}
