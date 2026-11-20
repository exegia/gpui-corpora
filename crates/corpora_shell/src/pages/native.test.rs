use super::{Completion, NativePage};
use corpora_services::native_corpus::Passage;
use gpui::{AppContext, Entity, TestAppContext, WindowOptions};

fn setup(cx: &mut TestAppContext) -> Entity<NativePage> {
    cx.update(|cx| {
        gpui_kit::init(cx);
        corpora_ui::ely::init_for_tests(cx);
        gpui_kit::open_window(WindowOptions::default(), cx, |window, cx| {
            cx.new(|cx| NativePage::new(window, cx))
        })
        .unwrap()
        .1
    })
}

#[gpui::test]
fn stale_completion_cannot_clear_pending_state_or_replace_error(cx: &mut TestAppContext) {
    let view = setup(cx);
    view.update(cx, |view, cx| {
        let (old, _) = view.requests.begin();
        let (current, _) = view.requests.begin();
        view.busy = Some("Opening corpus…");
        view.complete(old, Err(anyhow::anyhow!("stale")), cx);
        assert_eq!(view.busy, Some("Opening corpus…"));
        assert!(view.error.is_none());
        view.complete(current, Err(anyhow::anyhow!("Cannot open corpus")), cx);
        assert!(view.busy.is_none());
        assert_eq!(view.error.as_deref(), Some("Cannot open corpus"));
    });
}

#[gpui::test]
fn cancelled_passage_does_not_replace_selection(cx: &mut TestAppContext) {
    let view = setup(cx);
    view.update(cx, |view, cx| {
        let passage = |node| Passage {
            node,
            label: format!("Verse {node}"),
            text: "Text".into(),
            previous: None,
            next: None,
        };
        let (first, _) = view.requests.begin();
        view.complete(first, Ok(Completion::Passage(passage(5))), cx);
        let (cancelled, _) = view.requests.begin();
        view.requests.cancel();
        view.complete(cancelled, Ok(Completion::Passage(passage(6))), cx);
        assert_eq!(view.passage.as_ref().unwrap().node, 5);
    });
}
