use corpora_ui::ely::{
    buttons::Button,
    settings::{SettingsRow, SettingsSection},
    shell::set_dock_badge,
};
use gpui::{App, IntoElement, WeakEntity};

use crate::shell::Workspace;

/// Window-chrome previews: the loading skeleton and the Dock badge.
pub fn section(workspace: WeakEntity<Workspace>, cx: &App) -> impl IntoElement + use<> {
    let badge = workspace
        .upgrade()
        .is_some_and(|entity| entity.read(cx).badge);
    let loading = workspace.clone();
    SettingsSection::new(crate::i18n::localize("settings_section_workspace"))
        .description(crate::i18n::localize("settings_workspace_desc"))
        .row(
            SettingsRow::new(crate::i18n::localize("settings_preview_loading")).control(
                Button::new("preview-loading", crate::i18n::localize("settings_preview")).on_click(
                    move |_, _, cx| {
                        loading
                            .update(cx, |workspace, cx| workspace.preview_loading(cx))
                            .ok();
                    },
                ),
            ),
        )
        .row(
            SettingsRow::new(crate::i18n::localize("settings_preview_dock_badge")).control(
                Button::new(
                    "toggle-badge",
                    crate::i18n::localize(if badge {
                        "settings_clear_dock_badge"
                    } else {
                        "settings_preview"
                    }),
                )
                .on_click(move |_, _, cx| {
                    workspace
                        .update(cx, |state, cx| {
                            let next = !state.badge;
                            match set_dock_badge(next.then_some("1")) {
                                Ok(()) => {
                                    state.badge = next;
                                    cx.notify();
                                }
                                Err(error) => tracing::warn!(
                                    target: "corpora::settings",
                                    %error,
                                    "dock badge unavailable"
                                ),
                            }
                        })
                        .ok();
                }),
            ),
        )
}
