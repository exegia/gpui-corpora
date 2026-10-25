//! Validate refraction and moving glass against a striped backdrop on Metal.

use std::{path::PathBuf, sync::Arc, time::Duration};

use corpora_ui::atoms::tab::{Tab, Tabs};
use corpora_ui::ely::theme::ActiveTheme;
use gpui::{
    AppContext, Context, ElementId, HeadlessAppContext, IntoElement, Render, Window, div,
    prelude::*, px, size,
};
use gpui_kit::test::TestWindowExt;

struct Preview {
    visible: bool,
    selected: usize,
}

impl Render for Preview {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .relative()
            .children((0..45).map(|stripe| {
                div()
                    .absolute()
                    .left(px(stripe as f32 * 8.))
                    .top_0()
                    .w(px(8.))
                    .h_full()
                    .bg(cx.theme().colors.chart[if stripe % 2 == 0 { 6 } else { 0 }])
            }))
            .when(self.visible, |root| {
                root.child(
                    div().absolute().top(px(32.)).w_full().child(
                        Tabs::new("glass-tabs")
                            .items([
                                Tab::new("General"),
                                Tab::new("Workspace"),
                                Tab::new("Developer"),
                            ])
                            .selected_index(self.selected),
                    ),
                )
            })
    }
}

fn main() -> gpui::Result<()> {
    let output = std::env::args_os()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("target/tabs-glass-preview"));
    std::fs::create_dir_all(&output)?;
    let mut cx = HeadlessAppContext::with_platform(
        gpui_kit::platform::current_platform(true).text_system(),
        Arc::new(corpora_ui::ely::Assets::before(gpui_kit::assets::Assets)),
        gpui_kit::platform::current_headless_renderer,
    );
    cx.update(|cx| {
        gpui_kit::init(cx);
        corpora_ui::ely::init_for_tests(cx);
        corpora_ui::styles::tokens::DefaultTheme::new().install(cx);
        corpora_ui::ely::theme::Theme::set_mode_now(corpora_ui::ely::theme::Mode::Light, cx);
        corpora_ui::styles::tokens::sync_kit_theme(cx);
        cx.set_reduce_motion(false);
    });
    let window = cx.open_window(size(px(360.), px(120.)), |_, cx| {
        cx.new(|_| Preview {
            visible: false,
            selected: 0,
        })
    })?;
    let handle = window.into();
    cx.update_window(handle, |_, window, cx| window.render_frame(cx))?;
    let baseline = cx.capture_screenshot(handle)?;
    baseline.save(output.join("backdrop.png"))?;
    cx.update_window(handle, |view, window, cx| {
        view.downcast::<Preview>()
            .expect("preview root")
            .update(cx, |view, cx| {
                view.visible = true;
                cx.notify();
            });
        window.render_frame(cx);
    })?;
    let initial = cx.update_window(handle, |_, window, cx| {
        window.render_frame(cx);
        window
            .find((ElementId::from("glass-tabs"), "selected-indicator"))
            .bounds()
    })?;
    let glass = cx.capture_screenshot(handle)?;
    glass.save(output.join("glass-light.png"))?;
    let scale = glass.width() as f32 / 360.;
    let mut refracted = 0;
    for y in ((f32::from(initial.top()) + 4.) * scale) as u32
        ..((f32::from(initial.bottom()) - 4.) * scale) as u32
    {
        for x in ((f32::from(initial.left()) + 4.) * scale) as u32
            ..((f32::from(initial.right()) - 4.) * scale) as u32
        {
            let before = baseline.get_pixel(x, y);
            let after = glass.get_pixel(x, y);
            let before_chroma = i16::from(before[0]) - i16::from(before[2]);
            let after_chroma = i16::from(after[0]) - i16::from(after[2]);
            if before_chroma.abs() > 20
                && after_chroma.abs() > 20
                && before_chroma.signum() != after_chroma.signum()
            {
                refracted += 1;
            }
        }
    }
    assert!(
        refracted > 20,
        "glass must displace backdrop stripes; found {refracted} changed samples"
    );
    assert!(
        glass.get_pixel(0, 0) == baseline.get_pixel(0, 0),
        "glass must preserve pixels outside its bounds"
    );
    let corner_x = ((f32::from(initial.left()) + 1.) * scale) as u32;
    let corner_y = ((f32::from(initial.top()) + 1.) * scale) as u32;
    assert!(
        glass.get_pixel(corner_x, corner_y) == baseline.get_pixel(corner_x, corner_y),
        "glass must preserve pixels outside its rounded corners"
    );
    cx.update_window(handle, |view, window, cx| {
        view.downcast::<Preview>()
            .expect("preview root")
            .update(cx, |view, cx| {
                view.selected = 2;
                cx.notify();
            });
        window.render_frame(cx);
    })?;
    cx.advance_clock(Duration::from_millis(80));
    let middle = cx.update_window(handle, |_, window, cx| {
        window.render_frame(cx);
        window
            .find((ElementId::from("glass-tabs"), "selected-indicator"))
            .bounds()
    })?;
    assert!(
        middle.left() > initial.left() && middle.left() < initial.left() + px(150.),
        "glass must follow the in-progress GPUI Kit spring"
    );
    cx.capture_screenshot(handle)?
        .save(output.join("glass-moving.png"))?;
    cx.update(|cx| {
        cx.set_reduce_motion(true);
        corpora_ui::ely::theme::Theme::set_mode_now(corpora_ui::ely::theme::Mode::Dark, cx);
    });
    cx.update_window(handle, |_, window, cx| {
        window.render_frame(cx);
        let settled = window
            .find((ElementId::from("glass-tabs"), "selected-indicator"))
            .bounds();
        assert_eq!(settled.left(), initial.left() + px(150.));
    })?;
    cx.capture_screenshot(handle)?
        .save(output.join("glass-dark.png"))?;
    println!(
        "Verified {refracted} refracted pixels, outside clipping, and animated glass at {}",
        output.display()
    );
    Ok(())
}
