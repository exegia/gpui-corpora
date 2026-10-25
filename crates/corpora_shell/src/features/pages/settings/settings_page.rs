#[allow(unused_imports)]
use super::*;

use gpui_kit::component::{ActiveTheme, TitleBar, setting::Settings};

pub struct SettingsPage {
    devtools: crate::shell::Devtools,
    focus: FocusHandle,
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
        window.focus(&focus, cx);
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
        let index = super::sections::section_index(section);
        self.select_index(index, window, cx);
    }

    fn select_index(&mut self, index: usize, window: &mut Window, cx: &mut Context<Self>) {
        if self.selected != index {
            self.selected = index;
            self.devtools
                .set_route(&format!("/settings/{}", SECTIONS[index].0), window, cx);
            cx.notify();
        }
    }
}

impl Render for SettingsPage {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        use corpora_ui::atoms::tab::{Tab, TabVariant, Tabs};
        use gpui_kit::base::TestSupportExt;

        let page = self.page();
        let key = SECTIONS[self.selected].0;
        let theme = corpora_ui::ely::theme::ActiveTheme::theme(&**cx);
        let colors = theme.colors.clone();
        let dark = theme.is_dark();
        let background = colors.surface;
        let body = div()
            .flex()
            .flex_col()
            .flex_1()
            .min_h_0()
            .child(
                div().flex_none().pt(px(6.)).pb(px(10.)).px(px(7.)).child(
                    Tabs::new("settings-tabs")
                        .label(crate::i18n::localize("settings_title"))
                        .variant(TabVariant::Vertical)
                        .selected_index(self.selected)
                        .autofocus(self.focus.is_focused(window))
                        .items(SECTIONS.iter().enumerate().map(|(index, (_, name, icon))| {
                            let icon = gpui::svg()
                                .data(icon)
                                .text_color(if index == self.selected {
                                    colors.fg
                                } else {
                                    colors.fg.opacity(0.6)
                                })
                                .size(px(if index == 0 { 18. } else { 16. }))
                                .flex_none()
                                .id(SharedString::from(format!("settings-tab-icon-{index}")))
                                .test_support();
                            Tab::new(crate::i18n::localize(name)).icon(
                                div()
                                    .size(px(16.))
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .child(icon),
                            )
                        }))
                        .on_change(cx.listener(|this, index, window, cx| {
                            this.select_index(*index, window, cx);
                        })),
                ),
            )
            .child(
                div()
                    .id("settings-content")
                    .test_support()
                    .flex_1()
                    .min_h_0()
                    .mx(px(12.))
                    .mb(px(13.))
                    .rounded(px(12.))
                    .overflow_hidden()
                    .bg(background)
                    .border_1()
                    .border_color(colors.border)
                    .shadow(vec![
                        BoxShadow {
                            inset: true,
                            color: colors.shadow.opacity(0.25),
                            offset: point(px(0.), px(-1.)),
                            blur_radius: px(2.),
                            spread_radius: px(0.),
                        },
                        BoxShadow {
                            inset: true,
                            color: colors.on_media.opacity(if dark { 0.06 } else { 0.9 }),
                            offset: point(px(0.), px(2.)),
                            blur_radius: px(3.),
                            spread_radius: px(0.),
                        },
                    ])
                    .child(
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
            .relative()
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
            .rounded(px(15.))
            .overflow_hidden()
            .text_color(colors.fg)
            .child(corpora_ui::ui::layout::window_background(&colors))
            .child(
                TitleBar::new()
                    .relative()
                    .h(px(32.))
                    .pl_0()
                    .border_0()
                    .bg(gpui::transparent_black())
                    .child(
                        div()
                            .absolute()
                            .inset_0()
                            .flex()
                            .items_center()
                            .justify_center()
                            .gap(px(5.))
                            .text_size(px(13.))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child(
                                gpui::svg()
                                    .data(include_bytes!("assets/tabs/settings.svg"))
                                    .text_color(cx.theme().foreground)
                                    .size(px(15.)),
                            )
                            .child(crate::i18n::localize("settings_title")),
                    ),
            )
            .child(div().flex().flex_col().flex_1().min_h_0().child(body))
    }
}

gpui::actions!(settings_window, [CloseSettings]);
