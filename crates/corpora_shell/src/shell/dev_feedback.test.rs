use super::*;
use gpui::{Context, Modifiers, Render, TestAppContext, VisualTestContext, div, prelude::*, px};

struct Host {
    tools: Devtools,
    presses: usize,
}

impl Render for Host {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        self.tools.frame(
            "screen",
            "Screen",
            div().flex_1().min_h_0().child(
                div()
                    .id("app-button")
                    .size(px(40.))
                    .debug_selector(|| "app-button".into())
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.presses += 1;
                        cx.notify();
                    })),
            ),
            window,
            cx,
        )
    }
}

fn draw(cx: &mut VisualTestContext) {
    cx.run_until_parked();
    cx.update(|window, cx| window.draw(cx).clear(cx));
    cx.run_until_parked();
}

#[test]
fn settings_store_is_distinct_even_when_workspace_storage_is_overridden() {
    use std::path::Path;
    assert_eq!(
        settings_storage(Path::new("/tmp/custom.json")),
        Path::new("/tmp/custom-settings.json")
    );
    assert_eq!(
        settings_storage(Path::new("feedback")),
        Path::new("feedback-settings")
    );
}

#[gpui::test]
fn root_preserves_controls_and_saves_picked_comments_on_the_committed_route(
    cx: &mut TestAppContext,
) {
    cx.update(gpui_agentation::init_for_tests);
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("feedback.json");
    let (host, cx) = cx.add_window_view(|window, cx| Host {
        tools: Devtools {
            feedback: Some(
                gpui_agentation::Agentation::mount(
                    "Corpora test",
                    "gpui://corpora/home",
                    &path,
                    0,
                    window,
                    cx,
                )
                .unwrap(),
            ),
        },
        presses: 0,
    });
    draw(cx);
    let button = cx.debug_bounds("app-button").unwrap();
    cx.simulate_click(button.center(), Modifiers::none());
    draw(cx);
    cx.update(|_, cx| assert_eq!(host.read(cx).presses, 1));
    cx.update(|window, cx| {
        host.update(cx, |host, cx| host.tools.set_route("/projects", window, cx))
    });
    cx.simulate_keystrokes("tab");
    draw(cx);
    cx.simulate_event(gpui::KeyDownEvent {
        keystroke: gpui::Keystroke::parse("enter").unwrap(),
        is_held: false,
        prefer_character_input: false,
    });
    cx.simulate_event(gpui::KeyUpEvent {
        keystroke: gpui::Keystroke::parse("enter").unwrap(),
    });
    draw(cx);
    cx.simulate_mouse_move(button.center(), None, Modifiers::none());
    draw(cx);
    assert!(cx.debug_bounds("feedback-inspector-source").is_some());
    cx.simulate_click(button.center(), Modifiers::none());
    draw(cx);
    cx.update(|_, cx| assert_eq!(host.read(cx).presses, 1));
    cx.simulate_input("Corpora adapter feedback");
    draw(cx);
    let save = cx.debug_bounds("feedback-editor-primary").unwrap();
    cx.simulate_click(save.center(), Modifiers::none());
    draw(cx);
    let store = gpui_agentation::Store::open(&path).unwrap();
    let notes = store.app_annotations("Corpora test").unwrap();
    assert_eq!(notes.len(), 1);
    assert_eq!(notes[0].url, "gpui://corpora/projects");
    assert_eq!(notes[0].feedback.comment, "Corpora adapter feedback");
    assert_eq!(notes[0].feedback.element_path, "screen");
    cx.update(|window, cx| host.update(cx, |host, cx| host.tools.set_route("/home", window, cx)));
    draw(cx);
    assert_eq!(
        gpui_agentation::Store::open(&path)
            .unwrap()
            .app_annotations("Corpora test")
            .unwrap()
            .len(),
        1
    );
}
