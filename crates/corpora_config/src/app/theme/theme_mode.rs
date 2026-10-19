#[allow(unused_imports)]
use super::*;

/// Persisted light/dark choice; Ely's [`Mode`] carries no serde shape.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ThemeMode {
    #[default]
    Light,
    Dark,
}

impl From<Mode> for ThemeMode {
    fn from(mode: Mode) -> Self {
        match mode {
            Mode::Light => ThemeMode::Light,
            Mode::Dark => ThemeMode::Dark,
        }
    }
}
