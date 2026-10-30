#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn theme_mode_round_trips_through_ely_mode() {
    for mode in [ThemeMode::Light, ThemeMode::Dark] {
        assert_eq!(
            ThemeMode::from(mode.resolve(gpui::WindowAppearance::Light)),
            mode
        );
    }
}

#[test]
fn system_follows_os_appearance_while_explicit_choices_hold() {
    use gpui::WindowAppearance::{Dark, Light, VibrantDark, VibrantLight};
    for appearance in [Light, VibrantLight, Dark, VibrantDark] {
        let expected = if matches!(appearance, Dark | VibrantDark) {
            Mode::Dark
        } else {
            Mode::Light
        };
        assert_eq!(ThemeMode::System.resolve(appearance), expected);
        assert_eq!(ThemeMode::Light.resolve(appearance), Mode::Light);
        assert_eq!(ThemeMode::Dark.resolve(appearance), Mode::Dark);
        assert_eq!(
            ThemeMode::System.kit_mode(appearance).is_dark(),
            expected == Mode::Dark
        );
    }
}
