#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn test_config_roundtrip() {
    let original = AppConfig {
        version: 1,
        theme: "Gruvbox Dark".to_string(),
        scrollbar_show: None,
        locale: "en".to_string(),
        active_route: AppRoute::page(Page::Settings),
        sidebar_collapsed: true,
        native_notifications_enabled: false,
        global_shortcut_enabled: true,
        first_run_completed: true,
        notification_inbox: Vec::new(),
        window_bounds: Some(PersistedWindowBounds {
            x: 100.0,
            y: 200.0,
            width: 800.0,
            height: 600.0,
        }),
        granted_permissions: HashSet::from(["camera".to_string(), "microphone".to_string()]),
        denied_permissions: HashSet::new(),
        update_channel: "stable".to_string(),
        last_update_check: None,
        show_frame_time: false,
    };

    let json = serde_json::to_string(&original).expect("serialize config");
    let restored: AppConfig = serde_json::from_str(&json).expect("deserialize config");
    assert_eq!(original, restored, "round-tripped config must be identical");

    let mut snapshot = serde_json::to_value(&original).expect("config to value");
    for key in ["granted_permissions", "denied_permissions"] {
        if let Some(values) = snapshot.get_mut(key).and_then(|value| value.as_array_mut()) {
            values.sort_by(|left, right| left.as_str().cmp(&right.as_str()));
        }
    }
    insta::assert_yaml_snapshot!(
            &snapshot,
    @r#"
        version: 1
        theme: Gruvbox Dark
        scrollbar_show: ~
        locale: en
        active_route:
          Page: Settings
        sidebar_collapsed: true
        native_notifications_enabled: false
        global_shortcut_enabled: true
        first_run_completed: true
        notification_inbox: []
        window_bounds:
          x: 100
          y: 200
          width: 800
          height: 600
        granted_permissions:
          - camera
          - microphone
        denied_permissions: []
        update_channel: stable
        last_update_check: ~
        show_frame_time: false
        "#
        );
}
