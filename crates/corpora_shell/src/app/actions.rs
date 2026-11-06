use gpui::{Action, SharedString, actions};

use crate::app::theme::ThemeMode;

actions!(
    app,
    [
        About,
        Quit,
        ToggleSearch,
        OpenDiagnostics,
        TriggerTestPanic,
        Restart
    ]
);

mod execute_command;
mod select_locale;
mod switch_theme_mode;

pub use execute_command::ExecuteCommand;
pub use select_locale::SelectLocale;
pub use switch_theme_mode::SwitchThemeMode;
