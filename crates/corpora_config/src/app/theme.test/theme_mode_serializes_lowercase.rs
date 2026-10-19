#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn theme_mode_serializes_lowercase() {
    assert_eq!(serde_json::to_string(&ThemeMode::Dark).unwrap(), "\"dark\"");
    assert_eq!(
        serde_json::from_str::<ThemeMode>("\"light\"").unwrap(),
        ThemeMode::Light
    );
}
