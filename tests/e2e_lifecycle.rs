#[cfg(test)]
mod tests {
    // -----------------------------------------------------------------------
    // test_app_initializes
    // -----------------------------------------------------------------------

    /// Smoke test for the init sequence minus the GPUI event loop.
    #[test]
    fn test_app_initializes() {
        // Mirrors the `app::init` first step.
        gpui_corpora::lifecycle::install_panic_hook();

        // Marker under a throwaway data dir, as `app::init` installs.
        let data_dir = tempfile::tempdir().expect("tempdir");
        gpui_corpora::lifecycle::set_app_data_dir(data_dir.path().to_path_buf());

        // Crash marker write + read (mirrors startup crash detection).
        gpui_corpora::lifecycle::write_crash_marker();
        let marker = gpui_corpora::lifecycle::check_previous_crash();
        assert!(marker.is_some(), "crash marker should exist after write");

        // Clean removal (mirrors clean shutdown).
        gpui_corpora::lifecycle::remove_crash_marker();
        assert!(
            gpui_corpora::lifecycle::check_previous_crash().is_none(),
            "crash marker should be gone after removal"
        );
    }

    // -----------------------------------------------------------------------
    // test_single_instance_enforcement
    // -----------------------------------------------------------------------

    /// Same-named handles exclude each other without claiming the running app's lock.
    #[test]
    fn test_single_instance_enforcement() {
        #[cfg(target_os = "macos")]
        let lock_dir = tempfile::tempdir().unwrap();
        #[cfg(target_os = "macos")]
        let name = lock_dir
            .path()
            .join("instance")
            .to_string_lossy()
            .into_owned();
        #[cfg(not(target_os = "macos"))]
        let name = format!("com.corpora.tests.instance.{}", std::process::id());
        let first = single_instance::SingleInstance::new(&name).unwrap();
        assert!(
            first.is_single(),
            "first handle should acquire the isolated lock"
        );

        // Mutual exclusion: a same-named handle must not be the sole instance.
        let second = single_instance::SingleInstance::new(&name).unwrap();
        assert!(
            !second.is_single(),
            "second instance with the same name should not be considered single"
        );
        drop(second);
        drop(first);
        assert!(
            single_instance::SingleInstance::new(&name)
                .unwrap()
                .is_single(),
            "the lock should be available after its owner drops"
        );
    }

    // -----------------------------------------------------------------------
    // test_preflight_argv_accepts_quoted_command_line
    // -----------------------------------------------------------------------

    /// Explorer launches protocol URLs via the registry command
    /// `"...corpora.exe" "corpora://settings"`; the preflight argv
    /// scan must pick the URL out of that quoted shape and never mistake
    /// argv[0] (the exe path) for a link.
    #[test]
    fn test_preflight_argv_accepts_quoted_command_line() {
        let args = vec![
            "C:\\Program Files\\GPUI Starter\\corpora.exe".to_string(),
            "corpora://settings".to_string(),
        ];
        assert_eq!(
            gpui_corpora::single_instance::initial_deep_link(&args).as_deref(),
            Some("corpora://settings"),
            "quoted protocol launch must yield the URL argument"
        );

        let no_link = vec!["C:\\Program Files\\GPUI Starter\\corpora.exe".to_string()];
        assert_eq!(
            gpui_corpora::single_instance::initial_deep_link(&no_link),
            None,
            "a plain launch must not report a deep link"
        );
    }

    // -----------------------------------------------------------------------
    // test_config_loads
    // -----------------------------------------------------------------------

    /// `AppConfig::default()` is sensible and `normalized()` preserves it.
    #[test]
    fn test_config_loads() {
        use gpui_corpora::app_state::AppConfig;

        let config = AppConfig::default();

        assert_eq!(config.version, 1, "default config version should be 1");
        assert_eq!(config.theme, gpui_corpora::app::theme::ThemeMode::Light);
        assert_eq!(config.locale, "en");
        assert!(!config.sidebar_collapsed);
        assert!(config.global_shortcut_enabled);
        assert!(!config.first_run_completed);
        assert!(config.window_bounds.is_none());

        // normalized() must not mutate valid defaults.
        let normalized = config.clone().normalized();
        assert_eq!(config, normalized);
    }

    // -----------------------------------------------------------------------
    // test_storage_initializes
    // -----------------------------------------------------------------------

    /// SQLite opens, migrates, and answers queries outside the GPUI runtime.
    #[test]
    fn test_storage_initializes() {
        let dir = tempfile::tempdir().expect("tempdir");
        let db_path = dir.path().join("app.db");

        let conn = rusqlite::Connection::open(&db_path).expect("open connection");
        let version = gpui_corpora::db_migrations::run_migrations(&conn).expect("run migrations");
        assert_eq!(version, 3, "migrations should bring schema to version 3");

        // Health check.
        let one: i64 = conn
            .query_row("SELECT 1", [], |row| row.get(0))
            .expect("health check query");
        assert_eq!(one, 1);

        // Schema version is readable.
        let stored: i64 = conn
            .query_row(
                "SELECT COALESCE(MAX(version), 0) FROM schema_migrations",
                [],
                |row| row.get(0),
            )
            .expect("read schema version");
        assert_eq!(stored, 3);
    }
}
