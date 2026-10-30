#[allow(unused_imports)]
use super::*;

// The debounce task runs on the test scheduler, so these use the async
// harness and hold the App borrow only for each cx.update call.
#[gpui::test]
pub(crate) async fn update_config_persists_after_the_debounce_window(cx: &TestAppContext) {
    let _guard = debounce_test_lock();
    let dir = tempdir().unwrap();
    let state_file = dir.path().join("state.json");
    cx.update(|cx| cx.set_global(clean_state(&state_file)));

    cx.update(|cx| update_config(cx, |config| config.sidebar_collapsed = true));

    assert!(
        !state_file.exists(),
        "the write must wait for the debounce window to close"
    );
    cx.executor()
        .advance_clock(std::time::Duration::from_millis(DEBOUNCE_MS));
    cx.executor().run_until_parked();

    let loaded = read_persisted(&state_file);
    assert!(loaded.sidebar_collapsed);
    assert!(!cx.read(|cx| cx.global::<AppState>().dirty));
}
