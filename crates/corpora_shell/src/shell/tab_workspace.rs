use super::*;
use gpui::ScrollHandle;
use gpui_kit::component::resizable::ResizableState;

#[derive(Clone)]
pub(crate) struct TabWorkspace {
    pub pages: Pages,
    pub error_page: Option<Entity<RenderErrorPage>>,
    pub sidebar_visible: bool,
    pub inspector_visible: bool,
    pub inspector_width: gpui::Rems,
    pub split: Entity<ResizableState>,
    pub scroll: ScrollHandle,
    pub sidebar_scroll: ScrollHandle,
    pub inspector_scroll: ScrollHandle,
}

impl TabWorkspace {
    pub fn new(workspace: WeakEntity<Workspace>, window: &mut Window, cx: &mut App) -> Self {
        Self {
            pages: Pages {
                settings: cx.new(|cx| SettingsPage::new(workspace, window, cx)),
                notifications: cx.new(|cx| NotificationsPage::new(window, cx)),
                diagnostics: cx.new(|cx| DiagnosticsPage::new(window, cx)),
                about: cx.new(|_| AboutPage::new()),
            },
            error_page: None,
            sidebar_visible: true,
            inspector_visible: true,
            inspector_width: gpui::rems(20.0),
            split: cx.new(|_| ResizableState::default()),
            scroll: ScrollHandle::new(),
            sidebar_scroll: ScrollHandle::new(),
            inspector_scroll: ScrollHandle::new(),
        }
    }
}

gpui::actions!(workspace_panels, [ToggleSidebar, ToggleInspector]);
