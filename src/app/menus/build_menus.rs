#[allow(unused_imports)]
use super::*;

pub(crate) fn build_menus(title: SharedString, cx: &App) -> Vec<Menu> {
    let appearance = crate::app::theme::current_mode(cx);
    vec![
        Menu {
            name: title,
            items: vec![
                MenuItem::action(crate::i18n::localize("menu_about"), About),
                MenuItem::Separator,
                MenuItem::Submenu(Menu {
                    name: crate::i18n::localize("menu_appearance").into(),
                    items: vec![
                        MenuItem::action(
                            crate::i18n::localize("menu_appearance_light"),
                            SwitchThemeMode(ThemeMode::Light),
                        )
                        .checked(appearance == ThemeMode::Light),
                        MenuItem::action(
                            crate::i18n::localize("menu_appearance_dark"),
                            SwitchThemeMode(ThemeMode::Dark),
                        )
                        .checked(appearance == ThemeMode::Dark),
                        MenuItem::action(
                            crate::i18n::localize("menu_appearance_system"),
                            SwitchThemeMode(ThemeMode::System),
                        )
                        .checked(appearance == ThemeMode::System),
                    ],
                    disabled: false,
                }),
                language_menu(),
                MenuItem::Separator,
                MenuItem::action(crate::i18n::localize("menu_quit"), Quit),
            ],
            disabled: false,
        },
        Menu {
            name: crate::i18n::localize("menu_edit").into(),
            items: vec![
                MenuItem::action(
                    crate::i18n::localize("menu_edit_undo"),
                    ExecuteCommand(CommandId::Undo),
                )
                .disabled(!commands::availability(CommandId::Undo, cx).enabled),
                MenuItem::action(
                    crate::i18n::localize("menu_edit_redo"),
                    ExecuteCommand(CommandId::Redo),
                )
                .disabled(!commands::availability(CommandId::Redo, cx).enabled),
                MenuItem::separator(),
                MenuItem::action(crate::i18n::localize("menu_edit_cut"), Cut),
                MenuItem::action(crate::i18n::localize("menu_edit_copy"), Copy),
                MenuItem::action(crate::i18n::localize("menu_edit_paste"), Paste),
                MenuItem::separator(),
                MenuItem::action(crate::i18n::localize("menu_edit_select_all"), SelectAll),
            ],
            disabled: false,
        },
        Menu {
            name: crate::i18n::localize("menu_window").into(),
            items: vec![
                MenuItem::action(
                    crate::i18n::localize("menu_window_toggle_search"),
                    crate::app::ToggleSearch,
                ),
                MenuItem::action(
                    crate::i18n::localize("menu_window_diagnostics"),
                    ExecuteCommand(CommandId::OpenDiagnostics),
                ),
            ],
            disabled: false,
        },
        Menu {
            name: crate::i18n::localize("menu_go").into(),
            items: command_menu_items(
                &[
                    CommandId::OpenHome,
                    CommandId::OpenSettings,
                    CommandId::OpenNotifications,
                    CommandId::OpenDiagnostics,
                    CommandId::OpenAbout,
                    CommandId::CopyDiagnostics,
                    CommandId::OpenLogsFolder,
                    CommandId::OpenConfigFolder,
                ],
                cx,
            ),
            disabled: false,
        },
        Menu {
            name: crate::i18n::localize("menu_tools").into(),
            items: command_menu_items(
                &[CommandId::StartDemoTask, CommandId::CheckConnectivity],
                cx,
            ),
            disabled: false,
        },
    ]
}
