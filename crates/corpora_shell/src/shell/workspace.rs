#[allow(unused_imports)]
use super::*;

pub struct Workspace {
    pub(super) devtools: super::Devtools,
    pub(crate) focus: FocusHandle,
    pub(crate) dock: Entity<Dock>,
    pub(crate) toaster: Entity<Toaster>,
    pub(crate) tabs: WorkspaceTabs<super::tab_workspace::TabWorkspace>,
    routing_to: Option<String>,
    pub tray: Option<TrayIcon>,
    pub dock_visible: bool,
    pub loading: bool,
    pub badge: bool,
    pub palette_open: bool,
    pub project_statuses: HashMap<String, Status>,
    /// Applied once the routes have rendered: router hooks need a laid-out tree.
    pub(crate) pending_path: Option<String>,
    /// Last path written to config, so unchanged frames skip the write.
    pub(crate) persisted_path: String,
    pub(crate) _subscriptions: Vec<Subscription>,
}

impl Workspace {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let focus = cx.focus_handle();
        window.focus(&focus, cx);
        let this = cx.entity().downgrade();
        cx.set_global(WorkspaceHandle(this.clone()));

        let toaster = cx.new(|_| Toaster::default());
        crate::ui::toasts::install(toaster.clone(), cx);

        let dock = Self::new_dock(this.clone(), cx);

        let mut subscriptions = vec![
            cx.observe_window_appearance(window, |_, _, cx| {
                if crate::app::theme::current_mode(cx) == crate::app::theme::ThemeMode::System {
                    crate::app::theme::set_theme_mode_with_record(
                        crate::app::theme::ThemeMode::System,
                        false,
                        cx,
                    );
                }
            }),
            cx.observe_global_in::<events::AppEventQueue>(window, |this, _, cx| {
                this.drain_events(cx);
            }),
            // update_config already debounces; hand it the latest bounds directly.
            cx.observe_window_bounds(window, |_, window, cx| {
                let persisted =
                    crate::app::window::persisted_bounds(window.window_bounds().get_bounds());
                crate::app_state::update_config(cx, |config| {
                    config.window_bounds = Some(persisted);
                });
            }),
        ];
        subscriptions.extend([
            notify_on::<crate::tasks::TaskRegistry>(cx),
            notify_on::<crate::notifications::inbox::NotificationInboxState>(cx),
            notify_on::<crate::connectivity::ConnectivitySnapshot>(cx),
            notify_on::<crate::session::SessionSnapshot>(cx),
            notify_on::<crate::app_state::AppState>(cx),
            notify_on::<crate::app::window::SettingsWindowState>(cx),
        ]);

        cx.bind_keys([
            KeyBinding::new("secondary-b", super::tab_workspace::ToggleSidebar, None),
            KeyBinding::new(
                "secondary-shift-b",
                super::tab_workspace::ToggleInspector,
                None,
            ),
        ]);
        cx.bind_keys(
            (0..navigation::SECTIONS.len().min(9))
                .map(|i| KeyBinding::new(&crate::app::keys::page(i), NavigateToSection(i), None)),
        );

        let mut active_path = crate::app_state::with_config(cx, |c| c.active_path.clone());
        if active_path.split('/').nth(1) == Some("settings") {
            let path = active_path.clone();
            cx.defer(move |cx| {
                if let Err(error) =
                    crate::app::window::open_settings_window(path.split('/').nth(2), cx)
                {
                    tracing::error!(%error, "could not open Settings window");
                }
            });
            active_path = navigation::HOME.to_string();
            crate::app_state::update_config(cx, |config| config.active_path = active_path.clone());
        }
        if !cfg!(debug_assertions) && active_path.split('/').nth(1) == Some("ui") {
            active_path = navigation::HOME.to_string();
        }
        let devtools = super::Devtools::mount(&active_path, false, window, cx);
        let mut tabs = WorkspaceTabs::default();
        #[cfg(debug_assertions)]
        tabs.pin(
            "/ui",
            super::tab_workspace::TabWorkspace::new("/ui", window, cx),
        );
        tabs.open(
            &active_path,
            super::tab_workspace::TabWorkspace::new(&active_path, window, cx),
        );
        // Startup deep links were queued before this window existed.
        let drain = cx.entity().downgrade();
        cx.defer(move |cx| {
            drain.update(cx, |this, cx| this.drain_events(cx)).ok();
        });
        cx.spawn(async move |workspace, cx| {
            cx.background_executor()
                .timer(Duration::from_millis(450))
                .await;
            workspace
                .update(cx, |workspace, cx| {
                    workspace.loading = false;
                    cx.notify();
                })
                .ok();
        })
        .detach();

        Self {
            devtools,
            focus,
            dock,
            toaster,
            tabs,
            routing_to: Some(active_path.clone()),
            tray: None,
            dock_visible: false,
            loading: true,
            badge: false,
            palette_open: false,
            project_statuses: HashMap::new(),
            pending_path: (active_path != navigation::HOME).then_some(active_path.clone()),
            persisted_path: active_path,
            _subscriptions: subscriptions,
        }
    }

    pub(crate) fn new_dock(workspace: WeakEntity<Self>, cx: &mut Context<Self>) -> Entity<Dock> {
        cx.new(|cx| {
            let mut dock = Dock::new(
                move |_, _| routes::view(workspace.clone()).into_any_element(),
                |_, _, _| {
                    EmptyState::new(
                        "workspace-events",
                        IconName::Activity,
                        "No workspace events",
                    )
                    .body("Background activity will appear here when services are connected.")
                    .into_any_element()
                },
                cx,
            );
            dock.add(
                DockPanel {
                    id: "activity".into(),
                    title: "Activity".into(),
                    icon: IconName::Activity,
                },
                DockSide::Bottom,
            );
            dock
        })
    }

    /// Navigate after the routes next render. A tab path selects its tab; any
    /// other path shows in the route view.
    pub fn navigate(&mut self, path: impl Into<String>, cx: &mut Context<Self>) {
        let path = path.into();
        if path.split('/').nth(1) == Some("settings") {
            cx.defer(move |cx| {
                if let Err(error) =
                    crate::app::window::open_settings_window(path.split('/').nth(2), cx)
                {
                    tracing::error!(%error, "could not open Settings window");
                }
            });
            return;
        }
        self.tabs.show(&path);
        if let Some(tab) = self.tabs.active_mut() {
            tab.state.panels.navigate(&path);
        }
        self.routing_to = Some(path.clone());
        self.pending_path = Some(path);
        cx.notify();
    }

    fn tab_mut(&mut self) -> Option<&mut super::tab_workspace::TabWorkspace> {
        self.tabs.active_mut().map(|tab| &mut tab.state)
    }

    pub(crate) fn pages(&self) -> Option<&Pages> {
        self.tabs.active().map(|tab| &tab.state.pages)
    }

    pub(crate) fn new_tab(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let state = super::tab_workspace::TabWorkspace::new(EMPTY_TAB_PREFIX, window, cx);
        let path = self.tabs.new_empty(state);
        self.navigate(path, cx);
    }

    pub(crate) fn close_tab(&mut self, id: &str, cx: &mut Context<Self>) {
        if let Some(path) = self.tabs.close(id) {
            self.navigate(path, cx);
        } else if self.tabs.active().is_none() {
            self.pending_path = None;
            self.routing_to = None;
        }
        cx.notify();
    }

    pub fn toggle_palette(&mut self, cx: &mut Context<Self>) {
        self.palette_open = !self.palette_open;
        cx.notify();
    }

    pub fn preview_loading(&mut self, cx: &mut Context<Self>) {
        self.loading = true;
        cx.notify();
        cx.spawn(async move |workspace, cx| {
            cx.background_executor()
                .timer(Duration::from_millis(900))
                .await;
            workspace
                .update(cx, |workspace, cx| {
                    workspace.loading = false;
                    cx.notify();
                })
                .ok();
        })
        .detach();
    }

    pub(crate) fn open_deep_link(&mut self, link: &str, cx: &mut Context<Self>) {
        match navigation::parse_deep_link(link) {
            Ok(path) => self.navigate(path, cx),
            Err(err) => events::emit_error(err, cx),
        }
    }

    pub(crate) fn drain_events(&mut self, cx: &mut Context<Self>) {
        for event in events::drain(cx) {
            match event.kind {
                AppEventKind::Navigate(path) => self.navigate(path, cx),
                AppEventKind::DeepLinkReceived(link) => self.open_deep_link(&link, cx),
                AppEventKind::AppError { message, severity } => {
                    tracing::warn!(target: "corpora::workspace", error = %message, ?severity, "app error event received");
                    crate::error_surface::report(
                        message,
                        severity,
                        crate::error_surface::ErrorCategory::System,
                        vec![crate::error_surface::ErrorAction::Dismiss],
                        cx,
                    );
                    cx.notify();
                }
                AppEventKind::DiagnosticsChanged => {}
                AppEventKind::Test { message } => {
                    tracing::info!(target: "corpora::workspace", message, "test event received");
                }
                AppEventKind::RemoteCommand(cmd) => {
                    // Forwarded from a second instance over IPC.
                    use crate::platform::ipc::command::ForwardedCommand;
                    match cmd {
                        ForwardedCommand::TogglePalette => self.toggle_palette(cx),
                        ForwardedCommand::DeepLink(link) => self.open_deep_link(&link, cx),
                        ForwardedCommand::ShowWindow
                        | ForwardedCommand::HideWindow
                        | ForwardedCommand::Quit
                        | ForwardedCommand::ReloadConfig => {
                            tracing::debug!(
                                target: "corpora::workspace",
                                cmd = %cmd.label(),
                                "remote command forwarded (no handler)"
                            );
                        }
                    }
                }
            }
        }
    }

    /// The error page while the boundary is up, after taking any render
    /// panic recorded since the last frame.
    pub(crate) fn error_view(&mut self, cx: &mut Context<Self>) -> Option<AnyView> {
        let tab = self.tab_mut()?;
        if crate::lifecycle::take_render_panic() {
            tracing::warn!(target: "corpora::workspace", "render panic detected; activating error boundary");
            let summary = crate::lifecycle::last_panic_summary()
                .unwrap_or_else(|| crate::i18n::localize("render_error_unknown"));
            tab.error_page = Some(cx.new(|_| RenderErrorPage::new(summary)));
        }
        tab.error_page.clone().map(Into::into)
    }

    /// Persist the routed path once it changes; leaving a path also clears
    /// the error boundary.
    pub(crate) fn path_rendered(
        &mut self,
        path: &str,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.tabs.active().is_none() {
            return;
        }
        if self
            .routing_to
            .as_deref()
            .is_some_and(|target| target != path)
        {
            return;
        }
        self.routing_to = None;
        self.tabs.navigated(path);
        if let Some(tab) = self.tabs.active_mut() {
            tab.state.panels.navigate(path);
        }
        if self.persisted_path == path {
            return;
        }
        self.persisted_path = path.to_string();
        self.devtools.set_route(path, window, cx);
        if let Some(tab) = self.tab_mut() {
            tab.error_page = None;
        }
        let path = path.to_string();
        crate::app_state::update_config(cx, |config| config.active_path = path);
        cx.notify();
    }

    pub(crate) fn palette(&self, cx: &mut Context<Self>) -> CommandPalette {
        let this = cx.entity().downgrade();
        let close = this.clone();
        let commands = crate::commands::registry()
            .into_iter()
            .filter(|spec| crate::commands::availability(spec.id, cx).enabled)
            .map(|spec| Command::new(command_value(spec.id), spec.title).icon(spec.icon));
        CommandPalette::new("command-palette", move |_, cx| {
            close
                .update(cx, |workspace, cx| {
                    workspace.palette_open = false;
                    cx.notify();
                })
                .ok();
        })
        .group(crate::i18n::localize("palette_commands"), commands)
        .on_run(|value, window, cx| {
            let id = crate::commands::registry()
                .into_iter()
                .find(|spec| command_value(spec.id) == value.as_ref())
                .map(|spec| spec.id);
            match id {
                Some(id) => window.dispatch_action(Box::new(crate::app::ExecuteCommand(id)), cx),
                None => tracing::error!(target: "corpora::palette", %value, "unknown command"),
            }
        })
    }
}

impl Render for Workspace {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let started = std::time::Instant::now();
        if let Some(path) = self.pending_path.take() {
            let owner = cx.entity().downgrade();
            window.defer(cx, move |window, cx| {
                let navigate = owner
                    .update(cx, |this, cx| {
                        if this.routing_to.as_deref() != Some(path.as_str()) {
                            return false;
                        }
                        if !this.tabs.show(&path) {
                            let state = super::tab_workspace::TabWorkspace::new(&path, window, cx);
                            this.tabs.open(&path, state);
                            cx.notify();
                        }
                        true
                    })
                    .unwrap_or(false);
                if navigate {
                    use_navigate(window, cx).push(path);
                }
            });
        }
        let body = if self.dock_visible {
            self.dock.clone().into_any_element()
        } else {
            routes::view(cx.entity().downgrade()).into_any_element()
        };
        let status = status::bar(window, cx);
        let palette = self.palette_open.then(|| self.palette(cx));
        let theme = cx.theme();
        let background = window_background(&theme.colors);
        #[cfg(not(debug_assertions))]
        let scope = FocusScope::new(&self.focus).root();
        #[cfg(debug_assertions)]
        let scope = div().id("workspace-focus").track_focus(&self.focus);
        let scope = scope
            .size_full()
            .flex()
            .flex_col()
            .font_family(theme.font_family.clone())
            .text_size(theme.text_size(TextSize::Base))
            .text_color(theme.colors.fg)
            .child(div().flex_1().min_h_0().w_full().child(body))
            .child(super::dev_target("status-bar", "Status bar", status))
            .child(ToastViewport::new("toasts", &self.toaster))
            .children(palette);
        // Actions bubble from the focused element, so handlers sit outside the scope.
        let root = div()
            .relative()
            .w(window.viewport_size().width)
            .h(window.viewport_size().height)
            .on_action(cx.listener(
                |this, _: &super::tab_workspace::ToggleSidebar, window, cx| {
                    if let Some(tab) = this.tabs.active_mut()
                        && routes::panes(&tab.path).sidebar
                    {
                        tab.state.panels.sidebar_visible = !tab.state.panels.sidebar_visible;
                    }
                    window.focus(&this.focus, cx);
                    cx.notify();
                },
            ))
            .on_action(cx.listener(
                |this, _: &super::tab_workspace::ToggleInspector, window, cx| {
                    if let Some(tab) = this.tabs.active_mut()
                        && routes::panes(&tab.path).inspector
                    {
                        tab.state.panels.inspector_visible = !tab.state.panels.inspector_visible;
                    }
                    window.focus(&this.focus, cx);
                    cx.notify();
                },
            ))
            .on_action(
                cx.listener(|this, NavigateToSection(i): &NavigateToSection, _, cx| {
                    if let Some(path) = navigation::SECTIONS.get(*i) {
                        this.navigate(*path, cx);
                    }
                }),
            )
            .on_action(
                cx.listener(|this, _: &crate::app::ToggleSearch, _, cx| this.toggle_palette(cx)),
            )
            .on_action(cx.listener(|this, _: &ReloadCurrentPage, _, cx| {
                if let Some(tab) = this.tab_mut() {
                    tab.error_page = None;
                }
                cx.notify();
            }))
            .on_action(cx.listener(|this, action: &TriggerRenderError, _, cx| {
                let summary = action.message.clone();
                if let Some(tab) = this.tab_mut() {
                    tab.error_page = Some(cx.new(|_| RenderErrorPage::new(summary)));
                }
                cx.notify();
            }))
            .child(background)
            .child(scope);
        frame_time::store_frame_time(started.elapsed().as_micros() as u64);
        root
    }
}
