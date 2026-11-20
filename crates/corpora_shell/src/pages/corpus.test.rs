use super::*;
use gpui::{AppContext, Bounds, TestAppContext, WindowBounds, WindowOptions, px, size};

fn setup(cx: &mut TestAppContext) -> (gpui::AnyWindowHandle, Entity<CorpusPage>) {
    cx.update(|cx| {
        gpui_kit::init(cx);
        corpora_ui::ely::init_for_tests(cx);
        cx.set_global(CorpusSession::default());
        gpui_kit::open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(Bounds {
                    origin: Default::default(),
                    size: size(px(900.0), px(700.0)),
                })),
                ..Default::default()
            },
            cx,
            |window, cx| cx.new(|cx| CorpusPage::new(window, cx)),
        )
        .expect("open corpus page")
    })
}

#[gpui::test]
fn signed_out_session_clears_private_reader_state(cx: &mut TestAppContext) {
    let (window, page) = setup(cx);
    let other = cx
        .update_window(window, |_, window, cx| {
            cx.new(|cx| CorpusPage::new(window, cx))
        })
        .unwrap();
    cx.update(|cx| {
        for page in [&page, &other] {
            page.update(cx, |page, _| {
                page.loaded = true;
                page.corpora.push(Corpus {
                    id: Uuid::new_v4(),
                    title: "Private".into(),
                    state: "ready".into(),
                    language: None,
                });
                page.sections.push(Section {
                    reference: "Private section".into(),
                    child_count: 0,
                });
                page.passages.push(Passage {
                    node: 1,
                    reference: "Private reference".into(),
                    text: "Private text".into(),
                });
            });
        }
        cx.update_global::<CorpusSession, _>(|auth, _| auth.generation += 1);
    });
    cx.run_until_parked();
    cx.update(|cx| {
        for page in [&page, &other] {
            let page = page.read(cx);
            assert!(page.corpora.is_empty());
            assert!(page.sections.is_empty());
            assert!(page.passages.is_empty());
            assert!(!page.loaded);
        }
    });
}

#[gpui::test]
fn sign_in_masks_the_password_field(cx: &mut TestAppContext) {
    let (_, page) = setup(cx);
    cx.update(|cx| assert!(page.read(cx).password.read(cx).is_masked()));
}

#[gpui::test]
fn loading_more_text_keeps_the_retry_cursor(cx: &mut TestAppContext) {
    let (_, page) = setup(cx);
    cx.update(|cx| {
        page.update(cx, |page, _| {
            let corpus = Corpus {
                id: Uuid::new_v4(),
                title: "Example".into(),
                state: "ready".into(),
                language: None,
            };
            page.next_offset = Some(50);
            page.begin_content_load(corpus.clone(), None, true);
            assert_eq!(page.next_offset, Some(50));
            page.begin_content_load(corpus, None, false);
            assert_eq!(page.next_offset, None);
        });
    });
}
