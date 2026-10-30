#[allow(unused_imports)]
use super::*;

/// Persisted application configuration. The pub fields are the on-disk serde
/// schema pinned by tests/snapshot_tests.rs: add new fields with a
/// `#[serde(default)]`, never rename or repurpose an existing one.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AppConfig {
    pub version: u32,
    pub theme: ThemeMode,
    pub locale: String,
    /// Last gpui-router path, restored on launch.
    pub active_path: String,
    pub sidebar_collapsed: bool,
    pub native_notifications_enabled: bool,
    #[serde(default = "default_true")]
    pub global_shortcut_enabled: bool,
    pub first_run_completed: bool,
    pub notification_inbox: Vec<NotificationInboxItem>,
    pub window_bounds: Option<PersistedWindowBounds>,
    #[serde(default)]
    pub granted_permissions: HashSet<String>,
    #[serde(default)]
    pub denied_permissions: HashSet<String>,
    #[serde(default = "default_stable")]
    pub update_channel: String,
    #[serde(default)]
    pub last_update_check: Option<String>,
    #[serde(default = "default_show_frame_time")]
    pub show_frame_time: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            version: APP_STATE_VERSION,
            theme: ThemeMode::default(),
            locale: LOCALE_EN.to_string(),
            active_path: "/".to_string(),
            sidebar_collapsed: false,
            native_notifications_enabled: true,
            global_shortcut_enabled: true,
            first_run_completed: false,
            notification_inbox: Vec::new(),
            window_bounds: None,
            granted_permissions: HashSet::new(),
            denied_permissions: HashSet::new(),
            update_channel: default_stable(),
            last_update_check: None,
            show_frame_time: default_show_frame_time(),
        }
    }
}

impl AppConfig {
    /// Canonical locale/version, and drop window bounds a corrupt file could
    /// have planted so window creation always sees usable values.
    pub fn normalized(mut self) -> Self {
        if self.version == 0 {
            self.version = APP_STATE_VERSION;
        }
        if self.locale != LOCALE_EN && self.locale != LOCALE_ZH_CN {
            self.locale = LOCALE_EN.to_string();
        }
        if !self.active_path.starts_with('/') {
            self.active_path = "/".to_string();
        }
        if let Some(bounds) = &self.window_bounds
            && !bounds.is_plausible()
        {
            self.window_bounds = None;
        }
        self
    }
}
