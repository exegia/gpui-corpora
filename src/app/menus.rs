//! The native application menu bar, rebuilt when its checked or enabled state can change.

use ely_gpui_component::{
    forms::{Copy, Cut, Paste, SelectAll},
    theme::Theme,
};
use gpui_kit::{App, Menu, MenuItem, SharedString};

use crate::{
    app::{
        About, ExecuteCommand, LocaleState, Quit, SelectLocale, SwitchThemeMode, theme::ThemeMode,
    },
    commands::{self, CommandId},
};

mod build_menus;
mod command_menu_items;
mod init;
mod language_menu;

use build_menus::build_menus;
use command_menu_items::command_menu_items;
pub use init::init;
use language_menu::language_menu;
