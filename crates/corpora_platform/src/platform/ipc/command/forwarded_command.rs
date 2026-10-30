#[allow(unused_imports)]
use super::*;

/// A command a second instance can trigger in the primary process.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type", content = "payload")]
pub enum ForwardedCommand {
    ShowWindow,
    HideWindow,
    /// Toggle the command palette (or equivalent quick-action UI).
    TogglePalette,
    Quit,
    ReloadConfig,
    /// Open a deep link (`corpora://...`) in the primary instance.
    DeepLink(String),
}

impl ForwardedCommand {
    pub fn label(&self) -> &'static str {
        match self {
            Self::ShowWindow => "show_window",
            Self::HideWindow => "hide_window",
            Self::TogglePalette => "toggle_palette",
            Self::Quit => "quit",
            Self::ReloadConfig => "reload_config",
            Self::DeepLink(_) => "deep_link",
        }
    }
}
