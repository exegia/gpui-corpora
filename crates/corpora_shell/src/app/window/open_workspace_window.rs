#[allow(unused_imports)]
use super::*;

use ely_gpui_component::theme::ActiveTheme;

/// Open the workspace at its persisted bounds, or centered at a size that
/// fits the primary display.
pub fn open_workspace_window(title: &str, cx: &mut App) -> anyhow::Result<WindowHandle<Workspace>> {
    let mut window_size = size(px(1440.0), px(900.0));
    if let Some(display) = cx.primary_display() {
        let display_size = display.bounds().size;
        window_size.width = window_size.width.min(display_size.width * 0.85);
        window_size.height = window_size.height.min(display_size.height * 0.85);
    }
    let window_bounds = match crate::app_state::config(cx).window_bounds {
        Some(bounds) => Bounds {
            origin: point(px(bounds.x), px(bounds.y)),
            size: size(px(bounds.width), px(bounds.height)),
        },
        None => Bounds::centered(None, window_size, cx),
    };
    let mut controls = cx.theme().traffic_light_origin();
    controls.y += crate::shell::TITLEBAR_PADDING.to_pixels(cx.theme().base_rem());
    let options = WindowOptions {
        window_bounds: Some(WindowBounds::Windowed(window_bounds)),

        window_min_size: Some(Size {
            width: px(880.),
            height: px(600.),
        }),
        titlebar: Some(TitlebarOptions {
            title: Some(title.to_string().into()),
            appears_transparent: true,
            traffic_light_position: Some(controls),
        }),
        kind: WindowKind::Normal,
        // Becomes Wayland app_id / X11 WM_CLASS, matching the desktop entry.
        app_id: Some("corpora".to_string()),
        #[cfg(target_os = "linux")]
        window_background: gpui::WindowBackgroundAppearance::Transparent,
        #[cfg(target_os = "linux")]
        window_decorations: Some(gpui::WindowDecorations::Client),
        #[cfg(target_os = "linux")]
        icon: crate::app::assets::window_icon(),
        ..Default::default()
    };

    let handle = cx.open_window(options, |window, cx| {
        cx.new(|cx| Workspace::new(window, cx))
    })?;
    cx.set_global(RootWindow(handle.into()));
    // App level (not inside the window update) so the new window reports as
    // active in the accessibility snapshot.
    crate::accessibility::refresh(cx);
    Ok(handle)
}
