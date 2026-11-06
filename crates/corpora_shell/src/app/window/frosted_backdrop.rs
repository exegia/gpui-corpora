use gpui::{AnyWindowHandle, App, Styled, Window};
use gpui_kit::component::Theme;
use objc2::{MainThreadMarker, MainThreadOnly, rc::Retained};
use objc2_app_kit::{
    NSAppearance, NSAppearanceCustomization, NSAppearanceNameVibrantDark,
    NSAppearanceNameVibrantLight, NSAutoresizingMaskOptions, NSView, NSVisualEffectBlendingMode,
    NSVisualEffectMaterial, NSVisualEffectState, NSVisualEffectView, NSWindowOrderingMode,
};
use raw_window_handle::{HasWindowHandle, RawWindowHandle};

pub(super) fn install(handle: AnyWindowHandle, cx: &mut App) -> anyhow::Result<()> {
    handle.update(cx, |_, window, cx| {
        let material = install_material(window);
        gpui_kit::base::Root::update(window, cx, |root, window, cx| {
            root.style().background = Some(gpui::transparent_black().into());
            if let Some(material) = material {
                set_appearance(&material, cx);
                cx.observe_global_in::<Theme>(window, move |_, _, cx| {
                    set_appearance(&material, cx);
                })
                .detach();
            }
        });
    })
}

fn install_material(window: &Window) -> Option<Retained<NSVisualEffectView>> {
    let handle = HasWindowHandle::window_handle(window).ok()?;
    let RawWindowHandle::AppKit(handle) = handle.as_raw() else {
        return None;
    };
    let mtm = MainThreadMarker::new()?;
    // GPUI owns the AppKit view and keeps it alive for this Window borrow.
    let view = unsafe { &*handle.ns_view.as_ptr().cast::<NSView>() };
    // The live window owns this parent on the same AppKit thread.
    let parent = unsafe { view.superview() }?;
    let material = NSVisualEffectView::initWithFrame(NSVisualEffectView::alloc(mtm), view.frame());
    // GPUI's Blurred material removes AppKit's tint layers; retain them for a frosted window.
    material.setMaterial(NSVisualEffectMaterial::HUDWindow);
    material.setBlendingMode(NSVisualEffectBlendingMode::WithinWindow);
    material.setState(NSVisualEffectState::Active);
    let sizing =
        NSAutoresizingMaskOptions::ViewWidthSizable | NSAutoresizingMaskOptions::ViewHeightSizable;
    material.setAutoresizingMask(sizing);
    parent.addSubview_positioned_relativeTo(&material, NSWindowOrderingMode::Below, Some(view));
    Some(material)
}

fn set_appearance(material: &NSVisualEffectView, cx: &App) {
    // AppKit's immutable appearance-name constants are valid for the process lifetime.
    let name = unsafe {
        if Theme::global(cx).is_dark() {
            NSAppearanceNameVibrantDark
        } else {
            NSAppearanceNameVibrantLight
        }
    };
    material.setAppearance(NSAppearance::appearanceNamed(name).as_deref());
}
