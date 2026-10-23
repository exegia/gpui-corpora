#[allow(unused_imports)]
use super::*;

use gpui_kit::component::{
    ActiveTheme, TitleBar,
    setting::Settings,
    tab::{Tab, TabBar},
};

pub struct SettingsPage {
    devtools: crate::shell::Devtools,
    focus: FocusHandle,
    tab_focus: Vec<FocusHandle>,
    pub(crate) workspace: Option<WeakEntity<Workspace>>,
    pub(crate) selected: usize,
    pub(crate) notifications: NotificationRuntimeSnapshot,
    /// (event id, rendered description); the id is the row's stable identity,
    /// which the description string alone cannot provide.
    pub(crate) event_log: Vec<(EventId, String)>,
    pub(crate) _subscriptions: Vec<Subscription>,
}

impl SettingsPage {
    pub fn new(
        workspace: Option<WeakEntity<Workspace>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let focus = cx.focus_handle();
        let tab_focus: Vec<_> = SECTIONS.iter().map(|_| cx.focus_handle()).collect();
        window.focus(&tab_focus[0], cx);
        cx.bind_keys([
            KeyBinding::new("secondary-w", CloseSettings, Some("SettingsWindow")),
            #[cfg(not(target_os = "macos"))]
            KeyBinding::new("alt-f4", CloseSettings, Some("SettingsWindow")),
        ]);
        let mut subscriptions = vec![
            cx.observe_global_in::<crate::app_state::AppState>(window, |_, _, cx| cx.notify()),
            cx.observe_global_in::<LocaleState>(window, |_, window, cx| {
                window.set_window_title(&crate::i18n::localize("settings_title"));
                cx.notify();
            }),
            cx.observe_window_appearance(window, |_, _, cx| {
                if crate::app::theme::current_mode(cx) == crate::app::theme::ThemeMode::System {
                    crate::app::theme::set_theme_mode_with_record(
                        crate::app::theme::ThemeMode::System,
                        false,
                        cx,
                    );
                }
            }),
            cx.observe_global_in::<NativeNotificationState>(window, |this, _, cx| {
                this.notifications = notifications::snapshot(cx);
                cx.notify();
            }),
            cx.observe_global_in::<crate::events::AppEventQueue>(window, |this, window, cx| {
                // The workspace drains while open; keep log identity across observer calls.
                let mut appended = false;
                if let Some(queue) = cx.try_global::<crate::events::AppEventQueue>() {
                    for event in &queue.0 {
                        if this.event_log.iter().any(|(id, _)| *id == event.id) {
                            continue;
                        }
                        this.event_log
                            .push((event.id, format!("{:?} ({})", event.kind, event.id)));
                        appended = true;
                    }
                }
                let excess = this.event_log.len().saturating_sub(EVENT_LOG_CAP);
                this.event_log.drain(0..excess);
                if appended || excess > 0 {
                    cx.notify();
                }
                if this
                    .workspace
                    .as_ref()
                    .and_then(WeakEntity::upgrade)
                    .is_none()
                {
                    for path in pending_navigation::take_settings_requests(cx) {
                        if let Some(section) = path.split('/').nth(2) {
                            this.select_section(section, window, cx);
                        }
                        window.activate_window();
                    }
                }
            }),
        ];

        if let Some(owner) = workspace.as_ref().and_then(WeakEntity::upgrade) {
            subscriptions.push(cx.observe(&owner, |_, _, cx| cx.notify()));
            subscriptions.push(cx.observe_release(&owner, |_, _, cx| cx.notify()));
        }

        Self {
            devtools: crate::shell::Devtools::mount("/settings/general", true, window, cx),
            focus,
            tab_focus,
            workspace,
            selected: 0,
            notifications: notifications::snapshot(cx),
            event_log: Vec::new(),
            _subscriptions: subscriptions,
        }
    }

    pub(crate) fn select_section(
        &mut self,
        section: &str,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let index = SECTIONS
            .iter()
            .position(|(key, ..)| *key == section)
            .unwrap_or(0);
        self.select_index(index, window, cx);
    }

    fn select_index(&mut self, index: usize, window: &mut Window, cx: &mut Context<Self>) {
        window.focus(&self.tab_focus[index], cx);
        if self.selected != index {
            self.selected = index;
            self.devtools
                .set_route(&format!("/settings/{}", SECTIONS[index].0), window, cx);
            cx.notify();
        }
    }

    fn on_tab_key(&mut self, event: &KeyDownEvent, window: &mut Window, cx: &mut Context<Self>) {
        let next = match event.keystroke.key.as_str() {
            "left" => (self.selected + SECTIONS.len() - 1) % SECTIONS.len(),
            "right" => (self.selected + 1) % SECTIONS.len(),
            "home" => 0,
            "end" => SECTIONS.len() - 1,
            "enter" | "space" => self.selected,
            _ => return,
        };
        window.prevent_default();
        cx.stop_propagation();
        self.select_index(next, window, cx);
    }
}

impl Render for SettingsPage {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let page = self.page();
        let key = SECTIONS[self.selected].0;
        let ring = cx.theme().ring;
        let transparent = cx.theme().transparent;
        let body = div()
            .flex()
            .flex_col()
            .flex_1()
            .min_h_0()
            .child(
                div()
                    .p_4()
                    .on_key_down(cx.listener(Self::on_tab_key))
                    .child(
                        TabBar::new("settings-tabs")
                            .segmented()
                            .w_full()
                            .selected_index(self.selected)
                            .children(SECTIONS.iter().enumerate().map(|(index, (_, name, _))| {
                                Tab::new()
                                    .label(crate::i18n::localize(name))
                                    .flex_1()
                                    .track_focus(
                                        &self.tab_focus[index]
                                            .clone()
                                            .tab_stop(index == self.selected),
                                    )
                                    .border_1()
                                    .border_color(transparent)
                                    .focus_visible(move |style| style.border_color(ring))
                            }))
                            .on_click(cx.listener(|this, index, window, cx| {
                                this.select_index(*index, window, cx);
                            })),
                    ),
            )
            .child(
                div().flex_1().min_h_0().child(
                    // Segmented tabs own navigation; collapse the built-in sidebar.
                    Settings::new(SharedString::from(format!("settings-{key}")))
                        .sidebar_width(px(0.))
                        .sidebar_size_range(px(0.)..px(0.))
                        .sidebar_style(&StyleRefinement::default().hidden())
                        .header_style(&StyleRefinement::default().hidden())
                        .page(page),
                ),
            );
        let body = self
            .devtools
            .frame("settings-screen", "Settings", body, window, cx);
        div()
            .id("settings-window")
            .key_context("SettingsWindow")
            .track_focus(&self.focus)
            .on_action(|_: &CloseSettings, window, cx| {
                cx.stop_propagation();
                window.remove_window();
            })
            .size_full()
            .min_w_0()
            .flex()
            .flex_col()
            .bg(cx.theme().background)
            .text_color(cx.theme().foreground)
            .child(TitleBar::new().child(crate::i18n::localize("settings_title")))
            .child(div().flex_1().min_h_0().child(body))
    }
}

gpui::actions!(settings_window, [CloseSettings]);
