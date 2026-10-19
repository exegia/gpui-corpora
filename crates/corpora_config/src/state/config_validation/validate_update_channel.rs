#[allow(unused_imports)]
use super::*;

pub(crate) fn validate_update_channel(channel: &str) -> Vec<ConfigLint> {
    if channel.is_empty() {
        return vec![ConfigLint::new(
            "update_channel",
            LintSeverity::Warning,
            "update_channel is empty; the updater will default to \"stable\"",
        )];
    }
    if !KNOWN_UPDATE_CHANNELS.contains(&channel) {
        return vec![ConfigLint::new(
            "update_channel",
            LintSeverity::Warning,
            format!(
                "update_channel = {channel:?} is not recognised (expected one of {:?}); \
                 the updater will fall back to \"stable\"",
                KNOWN_UPDATE_CHANNELS
            ),
        )];
    }
    Vec::new()
}
