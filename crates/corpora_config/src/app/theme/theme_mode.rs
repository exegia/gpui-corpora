#[allow(unused_imports)]
use super::*;

/// Persisted appearance preference, including following the operating system.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ThemeMode {
    #[default]
    Light,
    Dark,
    System,
}

impl From<Mode> for ThemeMode {
    fn from(mode: Mode) -> Self {
        match mode {
            Mode::Light => ThemeMode::Light,
            Mode::Dark => ThemeMode::Dark,
        }
    }
}
