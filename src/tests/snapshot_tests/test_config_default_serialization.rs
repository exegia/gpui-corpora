#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn test_config_default_serialization() {
    let config = AppConfig::default();
    insta::assert_yaml_snapshot!(
            &config,
    @r#"
        version: 1
        theme: Default Light
        scrollbar_show: ~
        locale: en
        active_route:
          Page: Home
        sidebar_collapsed: false
        native_notifications_enabled: true
        global_shortcut_enabled: true
        first_run_completed: false
        notification_inbox: []
        window_bounds: ~
        granted_permissions: []
        denied_permissions: []
        update_channel: stable
        last_update_check: ~
        show_frame_time: true
        "#
        );
}
