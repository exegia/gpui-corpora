use gpui::{AppContext, Context, IntoElement, Render, Subscription, TestAppContext, Window, div};
use gpui_kit::test::TestWindowExt;

use super::{open_settings_window, settings_window_open};
use crate::app::{self, LocaleState};

struct LocalePreview {
    changes: usize,
    _locale: Subscription,
}

impl Render for LocalePreview {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
    }
}

#[gpui::test]
fn settings_reuses_its_window_and_keeps_global_changes_after_close(cx: &mut TestAppContext) {
    cx.update(|cx| {
        gpui_kit::init(cx);
        corpora_ui::ely::init_for_tests(cx);
        cx.set_reduce_motion(true);
        cx.set_global(LocaleState(app::LOCALE_EN.into()));
        crate::notifications::initialize(cx);
        cx.set_global(crate::events::AppEventQueue::default());
    });
    let (main, preview) = cx.update(|cx| {
        gpui_kit::open_window(Default::default(), cx, |_, cx| {
            cx.new(|cx: &mut Context<LocalePreview>| LocalePreview {
                changes: 0,
                _locale: cx.observe_global::<LocaleState>(|this, cx| {
                    this.changes += 1;
                    cx.notify();
                }),
            })
        })
        .unwrap()
    });
    let (settings, view) = cx.update(|cx| open_settings_window(None, cx).unwrap());
    cx.update_window(settings, |_, window, cx| {
        window.render_frame(cx);
        window.press("right", cx);
        assert_eq!(view.read(cx).selected, 1);
        window.press("home", cx);
        for width in [480., 541., 860.] {
            window.resize(gpui::size(gpui::px(width), gpui::px(493.)));
            window.render_frame(cx);
            let first = window.within("settings-tabs").find(0usize).bounds();
            let last = window.within("settings-tabs").find(3usize).bounds();
            assert_eq!(first.size, gpui::size(gpui::px(63.), gpui::px(53.)));
            assert_eq!(last.right() - first.left(), gpui::px(288.));
            assert!(
                (first.left() + last.right() - window.viewport_size().width).abs() < gpui::px(1.)
            );
            let panel = window.find("settings-content").bounds();
            assert_eq!(panel.top(), gpui::px(101.));
            assert_eq!(panel.size.height, gpui::px(379.));
            assert_eq!(panel.left(), gpui::px(12.));
            assert_eq!(panel.right(), window.viewport_size().width - gpui::px(12.));
            for index in 0..4 {
                let icon = window.find(gpui::SharedString::from(format!(
                    "settings-tab-icon-{index}"
                )));
                let side = gpui::px(if index == 0 { 18. } else { 16. });
                assert_eq!(icon.bounds().size, gpui::size(side, side));
            }
            assert!((last.size.width - first.size.width).abs() < gpui::px(1.));
        }
        for index in 1..4 {
            window.within("settings-tabs").click(index, cx);
            assert_eq!(view.read(cx).selected, index);
        }
        window.press("left", cx);
        assert_eq!(view.read(cx).selected, 2);
        window.press("right", cx);
        assert_eq!(view.read(cx).selected, 3);
        window.press("home", cx);
        assert_eq!(view.read(cx).selected, 0);
        window.press("end", cx);
        assert_eq!(view.read(cx).selected, 3);
        window.press("space", cx);
        assert_eq!(view.read(cx).selected, 3);
    })
    .unwrap();
    cx.update(|cx| {
        let (again, _) = open_settings_window(None, cx).unwrap();
        assert_eq!(again.window_id(), settings.window_id());
        assert_eq!(view.read(cx).selected, 3);
        let (_, selected) = open_settings_window(Some("notifications"), cx).unwrap();
        assert_eq!(selected.read(cx).selected, 0);
        assert_eq!(cx.windows().len(), 2);
        app::set_locale(app::LOCALE_ZH_CN, cx);
    });
    cx.update_window(settings, |_, _, cx| {
        crate::commands::execute(crate::commands::CommandId::OpenSettings, cx);
    })
    .unwrap();
    cx.run_until_parked();
    cx.update(|cx| {
        assert_eq!(cx.windows().len(), 2);
        assert_eq!(
            open_settings_window(None, cx).unwrap().0.window_id(),
            settings.window_id()
        );
        assert!(preview.read(cx).changes > 0);
        assert_eq!(app::current_locale(cx), app::LOCALE_ZH_CN);
        crate::events::emit(
            crate::events::AppEventKind::DeepLinkReceived("corpora://settings/storage".into()),
            cx,
        );
    });
    cx.run_until_parked();
    cx.update(|cx| assert_eq!(view.read(cx).selected, 3));
    cx.update(|cx| {
        crate::events::emit(
            crate::events::AppEventKind::Navigate("/projects".into()),
            cx,
        );
        crate::events::emit(
            crate::events::AppEventKind::RemoteCommand(
                crate::platform::ipc::command::ForwardedCommand::DeepLink(
                    "corpora://settings/workspace".into(),
                ),
            ),
            cx,
        );
    });
    cx.run_until_parked();
    cx.update(|cx| {
        assert_eq!(view.read(cx).selected, 1);
        assert!(cx.global::<crate::events::AppEventQueue>().0.iter().any(|event| {
            matches!(&event.kind, crate::events::AppEventKind::Navigate(path) if path == "/projects")
        }));
    });
    cx.update_window(settings, |_, window, cx| {
        window.render_frame(cx);
        window.press("secondary-w", cx);
    })
    .unwrap();
    cx.run_until_parked();
    let (_, reopened_view) = cx.update(|cx| {
        assert!(!settings_window_open(cx));
        assert_eq!(cx.windows().len(), 1);
        assert_eq!(cx.windows()[0].window_id(), main.window_id());
        assert_eq!(app::current_locale(cx), app::LOCALE_ZH_CN);
        let (reopened, selected) = open_settings_window(Some("storage"), cx).unwrap();
        assert_ne!(reopened.window_id(), settings.window_id());
        assert_eq!(selected.read(cx).selected, 3);
        app::set_locale(app::LOCALE_EN, cx);
        (reopened, selected)
    });
    cx.update_window(main, |_, window, _| window.remove_window())
        .unwrap();
    cx.run_until_parked();
    cx.update(|cx| {
        assert_eq!(cx.windows().len(), 1);
        crate::events::emit(
            crate::events::AppEventKind::DeepLinkReceived("corpora://settings/developer".into()),
            cx,
        );
    });
    cx.run_until_parked();
    cx.update(|cx| assert_eq!(reopened_view.read(cx).selected, 3));
}
