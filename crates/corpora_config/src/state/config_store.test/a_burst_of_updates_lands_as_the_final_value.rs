#[allow(unused_imports)]
use super::*;

#[gpui::test]
pub(crate) async fn a_burst_of_updates_lands_as_the_final_value(cx: &TestAppContext) {
    let _guard = debounce_test_lock();
    let dir = tempdir().unwrap();
    let state_file = dir.path().join("state.json");
    cx.update(|cx| cx.set_global(clean_state(&state_file)));

    cx.update(|cx| update_config(cx, |config| config.sidebar_collapsed = true));
    cx.update(|cx| {
        update_config(cx, |config| {
            config.theme = crate::app::theme::ThemeMode::Dark
        })
    });

    assert!(
        !state_file.exists(),
        "both mutations share one debounce window"
    );
    cx.executor()
        .advance_clock(std::time::Duration::from_millis(DEBOUNCE_MS));
    cx.executor().run_until_parked();

    let loaded = read_persisted(&state_file);
    assert!(loaded.sidebar_collapsed);
    assert_eq!(loaded.theme, crate::app::theme::ThemeMode::Dark);
}
