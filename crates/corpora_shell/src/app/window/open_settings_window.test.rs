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
        for width in [760., 860., 1100.] {
            window.resize(gpui::size(gpui::px(width), gpui::px(680.)));
            window.render_frame(cx);
            let first = window.within("settings-tabs").find(0usize).bounds();
            let last = window.within("settings-tabs").find(5usize).bounds();
            assert!(last.right() - first.left() > window.viewport_size().width * 0.9);
            assert!((last.size.width - first.size.width).abs() < gpui::px(1.));
        }
        for index in 1..6 {
            window.within("settings-tabs").click(index, cx);
            assert_eq!(view.read(cx).selected, index);
        }
        window.press("left", cx);
        assert_eq!(view.read(cx).selected, 4);
        window.press("right", cx);
        assert_eq!(view.read(cx).selected, 5);
        window.press("home", cx);
        assert_eq!(view.read(cx).selected, 0);
        window.press("end", cx);
        assert_eq!(view.read(cx).selected, 5);
        window.press("space", cx);
        assert_eq!(view.read(cx).selected, 5);
    })
    .unwrap();
    cx.update(|cx| {
        let (again, _) = open_settings_window(None, cx).unwrap();
        assert_eq!(again.window_id(), settings.window_id());
        assert_eq!(view.read(cx).selected, 5);
        let (_, selected) = open_settings_window(Some("notifications"), cx).unwrap();
        assert_eq!(selected.read(cx).selected, 1);
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
    cx.update(|cx| assert_eq!(view.read(cx).selected, 4));
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
        assert_eq!(view.read(cx).selected, 2);
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
        assert_eq!(selected.read(cx).selected, 4);
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
    cx.update(|cx| assert_eq!(reopened_view.read(cx).selected, 5));
}
