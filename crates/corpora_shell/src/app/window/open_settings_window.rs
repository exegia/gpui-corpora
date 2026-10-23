use gpui::{Subscription, WeakEntity};

use super::*;
use crate::views::SettingsPage;

/// The optional Settings window; preferences remain in their existing globals.
pub struct SettingsWindowState {
    window: Option<(AnyWindowHandle, WeakEntity<SettingsPage>)>,
    _closed: Subscription,
}

impl Global for SettingsWindowState {}

/// Whether the Settings window is open.
pub fn settings_window_open(cx: &App) -> bool {
    cx.try_global::<SettingsWindowState>()
        .is_some_and(|state| state.window.is_some())
}

/// Activate the existing Settings window or create it, optionally selecting a section.
pub fn open_settings_window(
    section: Option<&str>,
    cx: &mut App,
) -> anyhow::Result<(AnyWindowHandle, Entity<SettingsPage>)> {
    if !cx.has_global::<SettingsWindowState>() {
        let closed = cx.on_window_closed(|cx, id| {
            let matches = cx.try_global::<SettingsWindowState>().is_some_and(|state| {
                state
                    .window
                    .as_ref()
                    .is_some_and(|(handle, _)| handle.window_id() == id)
            });
            if matches {
                cx.global_mut::<SettingsWindowState>().window = None;
            }
        });
        cx.set_global(SettingsWindowState {
            window: None,
            _closed: closed,
        });
    }
    let existing = cx.global::<SettingsWindowState>().window.clone();
    if let Some((handle, weak)) = existing
        && let Some(view) = weak.upgrade()
        && handle
            .update(cx, |_, window, cx| {
                if let Some(section) = section {
                    view.update(cx, |view, cx| view.select_section(section, window, cx));
                }
                window.activate_window();
            })
            .is_ok()
    {
        return Ok((handle, view));
    }
    cx.global_mut::<SettingsWindowState>().window = None;
    let options = WindowOptions {
        window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
            None,
            size(px(860.), px(680.)),
            cx,
        ))),
        window_min_size: Some(size(px(760.), px(520.))),
        app_id: Some("corpora".to_string()),
        ..gpui_kit::component::TitleBar::window_options()
    };
    let workspace = crate::shell::current_workspace(cx);
    let (handle, view) = gpui_kit::open_window(options, cx, |window, cx| {
        window.set_window_title(&crate::i18n::localize("settings_title"));
        cx.new(|cx| {
            let mut view = SettingsPage::new(workspace, window, cx);
            if let Some(section) = section {
                view.select_section(section, window, cx);
            }
            view
        })
    })?;
    cx.global_mut::<SettingsWindowState>().window = Some((handle, view.downgrade()));
    Ok((handle, view))
}

#[cfg(test)]
#[path = "open_settings_window.test.rs"]
mod tests;
