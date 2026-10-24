//! Render settings with native text shaping and Metal, without opening desktop windows.

use std::{path::PathBuf, sync::Arc};

use corpora_config::app::{LOCALE_EN, LocaleState};
use corpora_shell::app::window::open_settings_window;
use gpui::{HeadlessAppContext, px, size};
use gpui_kit::test::TestWindowExt;

fn main() -> anyhow::Result<()> {
    let output = std::env::args_os()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("target/settings-preview"));
    std::fs::create_dir_all(&output)?;
    let platform = gpui_kit::platform::current_platform(true);
    let assets = corpora_ui::ely::Assets::before(gpui_kit::assets::Assets);
    let mut cx = HeadlessAppContext::with_platform(
        platform.text_system(),
        Arc::new(assets),
        gpui_kit::platform::current_headless_renderer,
    );
    cx.update(|cx| {
        gpui_kit::init(cx);
        corpora_ui::ely::init_for_tests(cx);
        cx.set_reduce_motion(true);
        corpora_ui::styles::tokens::MacosClassic::new().install(cx);
        cx.set_global(LocaleState(LOCALE_EN.into()));
        corpora_config::notifications::initialize(cx);
        cx.set_global(corpora_platform::events::AppEventQueue::default());
    });
    let (handle, _) = cx.update(|cx| open_settings_window(None, cx))?;
    for dark in [false, true] {
        cx.update(|cx| {
            corpora_ui::ely::theme::Theme::set_mode_now(
                if dark {
                    corpora_ui::ely::theme::Mode::Dark
                } else {
                    corpora_ui::ely::theme::Mode::Light
                },
                cx,
            );
            gpui_kit::component::Theme::change(
                if dark {
                    gpui_kit::component::ThemeMode::Dark
                } else {
                    gpui_kit::component::ThemeMode::Light
                },
                None,
                cx,
            );
        });
        for section in ["general", "workspace", "shortcuts", "developer"] {
            cx.update(|cx| open_settings_window(Some(section), cx))?;
            cx.update_window(handle, |_, window, cx| {
                window.resize(size(px(541.), px(493.)));
                window.render_frame(cx);
            })?;
            // Settle virtualized list measurements before capturing a theme change.
            for _ in 0..3 {
                cx.run_until_parked();
                cx.update_window(handle, |_, window, cx| window.render_frame(cx))?;
            }
            let path = output.join(format!(
                "settings-{}-{section}.png",
                if dark { "dark" } else { "light" }
            ));
            cx.capture_screenshot(handle)?.save(&path)?;
            println!("{}", path.display());
        }
    }
    Ok(())
}
