pub mod init;
pub mod menus;

pub use corpora_config::app::{locale, theme};
pub use corpora_platform::app::reload;
pub use corpora_services::app::lifecycle;
pub use corpora_shell::app::{actions, assets, keys, window};

pub use actions::{
    About, ExecuteCommand, OpenDiagnostics, Quit, Restart, SelectLocale, SwitchThemeMode,
    ToggleSearch, TriggerTestPanic,
};
pub use init::init;
pub use locale::{LOCALE_EN, LOCALE_ZH_CN, LocaleState, current_locale, set_locale};
pub use reload::{is_reload_requested, perform_reload, request_reload};
pub use theme::{set_theme_mode, set_theme_mode_with_record};
pub use window::open_workspace_window;
