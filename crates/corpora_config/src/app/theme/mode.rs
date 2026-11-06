use super::*;
use gpui::WindowAppearance;

impl ThemeMode {
    /// Resolve the saved preference against the current operating-system appearance.
    pub fn resolve(self, system: WindowAppearance) -> Mode {
        match self {
            Self::Light => Mode::Light,
            Self::Dark => Mode::Dark,
            Self::System => match system {
                WindowAppearance::Dark | WindowAppearance::VibrantDark => Mode::Dark,
                WindowAppearance::Light | WindowAppearance::VibrantLight => Mode::Light,
            },
        }
    }

    /// Resolve the same preference for the GPUI Kit components hosted beside Ely.
    pub fn kit_mode(self, system: WindowAppearance) -> gpui_kit::component::ThemeMode {
        match self.resolve(system) {
            Mode::Light => gpui_kit::component::ThemeMode::Light,
            Mode::Dark => gpui_kit::component::ThemeMode::Dark,
        }
    }
}
