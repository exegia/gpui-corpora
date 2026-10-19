#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn theme_mode_round_trips_through_ely_mode() {
    for mode in [ThemeMode::Light, ThemeMode::Dark] {
        assert_eq!(ThemeMode::from(Mode::from(mode)), mode);
    }
}
