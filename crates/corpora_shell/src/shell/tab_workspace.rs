use super::*;
use gpui::ScrollHandle;
use gpui_kit::component::resizable::ResizableState;

/// Pane state for one workspace tab, or for the view route pages share.
#[derive(Clone)]
pub(crate) struct TabWorkspace {
    pub pages: Pages,
    pub error_page: Option<Entity<RenderErrorPage>>,
    pub panels: super::pane_visibility::PaneVisibility,
    pub inspector_width: gpui::Rems,
    pub content_split: Entity<ResizableState>,
    pub dock_split: Entity<ResizableState>,
    pub scroll: ScrollHandle,
    pub sidebar_scroll: ScrollHandle,
    pub inspector_scroll: ScrollHandle,
}

impl TabWorkspace {
    pub fn new(path: &str, window: &mut Window, cx: &mut App) -> Self {
        Self {
            pages: Pages {
                #[cfg(debug_assertions)]
                ui: cx.new(|cx| crate::routes::ui::Ui::new(window, cx)),
                #[cfg(debug_assertions)]
                button: cx.new(|_| crate::routes::ui::button::ButtonShowcase::new()),
                #[cfg(debug_assertions)]
                glass: cx.new(crate::routes::ui::glass::GlassShowcase::new),
                notifications: cx.new(|cx| NotificationsPage::new(window, cx)),
                diagnostics: cx.new(|cx| DiagnosticsPage::new(window, cx)),
                about: cx.new(|_| AboutPage::new()),
            },
            error_page: None,
            panels: super::pane_visibility::PaneVisibility::new(path),
            inspector_width: gpui::rems(20.0),
            content_split: cx.new(|_| ResizableState::default()),
            dock_split: cx.new(|_| ResizableState::default()),
            scroll: ScrollHandle::new(),
            sidebar_scroll: ScrollHandle::new(),
            inspector_scroll: ScrollHandle::new(),
        }
    }
}

gpui::actions!(workspace_panels, [ToggleSidebar, ToggleInspector]);
