#[allow(unused_imports)]
use super::*;

pub struct Workspace {
    #[cfg(debug_assertions)]
    annotations: Option<Entity<gpui_agentation::AnnotationOverlay>>,
    pub(crate) focus: FocusHandle,
    pub(crate) dock: Entity<Dock>,
    pub(crate) toaster: Entity<Toaster>,
    pub pages: Pages,
    pub(crate) tabs: WorkspaceTabs,
    pub tray: Option<TrayIcon>,
    pub sidebar_visible: bool,
    pub dock_visible: bool,
    pub loading: bool,
    pub badge: bool,
    pub palette_open: bool,
    pub project_statuses: HashMap<String, Status>,
    /// Applied once the routes have rendered: router hooks need a laid-out tree.
    pub(crate) pending_path: Option<String>,
    /// Last path written to config, so unchanged frames skip the write.
    pub(crate) persisted_path: String,
    /// The error boundary replaces the routed page after a render panic until
    /// the next navigation or "Reload Page".
    pub(crate) error_page: Option<Entity<RenderErrorPage>>,
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

        let pages = Pages {
            settings: cx.new(|cx| SettingsPage::new(this.clone(), window, cx)),
            notifications: cx.new(|cx| NotificationsPage::new(window, cx)),
            diagnostics: cx.new(|cx| DiagnosticsPage::new(window, cx)),
            about: cx.new(|_| AboutPage::new()),
        };
        let dock = Self::new_dock(this, cx);

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
        ]);

        cx.bind_keys(
            (0..navigation::SECTIONS.len().min(9))
                .map(|i| KeyBinding::new(&crate::app::keys::page(i), NavigateToSection(i), None))
                .chain([KeyBinding::new(
                    crate::app::keys::TOGGLE_SIDEBAR,
                    ToggleSidebar,
                    None,
                )]),
        );

        let (active_path, collapsed) =
            crate::app_state::with_config(cx, |c| (c.active_path.clone(), c.sidebar_collapsed));
        #[cfg(debug_assertions)]
        let annotations = super::dev_feedback::mount(&active_path, window, cx);
        let mut tabs = WorkspaceTabs::default();
        tabs.restore_empty(&active_path);
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
            #[cfg(debug_assertions)]
            annotations,
            focus,
            dock,
            toaster,
            pages,
            tabs,
            tray: None,
            sidebar_visible: !collapsed,
            dock_visible: false,
            loading: true,
            badge: false,
            palette_open: false,
            project_statuses: HashMap::new(),
            pending_path: (active_path != navigation::HOME).then_some(active_path.clone()),
            persisted_path: active_path,
            error_page: None,
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

    /// Navigate after the routes next render.
    pub fn navigate(&mut self, path: impl Into<String>, cx: &mut Context<Self>) {
        self.pending_path = Some(path.into());
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

    /// Persisted so restarts keep the choice.
    pub(crate) fn toggle_sidebar(&mut self, cx: &mut Context<Self>) {
        self.sidebar_visible = !self.sidebar_visible;
        let collapsed = !self.sidebar_visible;
        crate::app_state::update_config(cx, |config| config.sidebar_collapsed = collapsed);
        cx.notify();
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
        if crate::lifecycle::take_render_panic() {
            tracing::warn!(target: "corpora::workspace", "render panic detected; activating error boundary");
            let summary = crate::lifecycle::last_panic_summary()
                .unwrap_or_else(|| crate::i18n::localize("render_error_unknown"));
            self.error_page = Some(cx.new(|_| RenderErrorPage::new(summary)));
        }
        self.error_page.clone().map(Into::into)
    }

    /// Persist the routed path once it changes; leaving a path also clears
    /// the error boundary.
    pub(crate) fn path_rendered(
        &mut self,
        path: &str,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        #[cfg(not(debug_assertions))]
        let _ = window;
        if self.persisted_path == path {
            return;
        }
        self.persisted_path = path.to_string();
        self.tabs.navigated(path);
        #[cfg(debug_assertions)]
        if let Some(annotations) = &self.annotations {
            annotations.update(cx, |overlay, cx| {
                overlay.set_route(&format!("gpui://corpora{path}"), window, cx)
            });
        }
        self.error_page = None;
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
        #[cfg(debug_assertions)]
        gpui_agentation::begin_frame(window, cx);
        let started = std::time::Instant::now();
        if let Some(path) = self.pending_path.take() {
            window.defer(cx, move |window, cx| use_navigate(window, cx).push(path));
        }
        let body = if self.dock_visible {
            self.dock.clone().into_any_element()
        } else {
            routes::view(cx.entity().downgrade()).into_any_element()
        };
        let status = status::bar(window, cx);
        let palette = self.palette_open.then(|| self.palette(cx));
        let theme = cx.theme();
        let background = window_background(theme.is_dark());
        let scope = FocusScope::new(&self.focus)
            .root()
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
            .on_action(
                cx.listener(|this, NavigateToSection(i): &NavigateToSection, _, cx| {
                    if let Some(path) = navigation::SECTIONS.get(*i) {
                        this.navigate(*path, cx);
                    }
                }),
            )
            .on_action(cx.listener(|this, _: &ToggleSidebar, _, cx| this.toggle_sidebar(cx)))
            .on_action(
                cx.listener(|this, _: &crate::app::ToggleSearch, _, cx| this.toggle_palette(cx)),
            )
            .on_action(cx.listener(|this, _: &ReloadCurrentPage, _, cx| {
                this.error_page = None;
                cx.notify();
            }))
            .on_action(cx.listener(|this, action: &TriggerRenderError, _, cx| {
                let summary = action.message.clone();
                this.error_page = Some(cx.new(|_| RenderErrorPage::new(summary)));
                cx.notify();
            }))
            .child(background)
            .child(scope);
        #[cfg(debug_assertions)]
        let root = root.children(self.annotations.clone());
        frame_time::store_frame_time(started.elapsed().as_micros() as u64);
        root
    }
}
