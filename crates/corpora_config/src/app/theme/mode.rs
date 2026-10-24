#[allow(unused_imports)]
use super::*;

impl From<ThemeMode> for Mode {
    fn from(mode: ThemeMode) -> Self {
        match mode {
            ThemeMode::Light => Mode::Light,
            ThemeMode::Dark => Mode::Dark,
        }
    }
}
