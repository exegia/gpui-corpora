#[allow(unused_imports)]
use super::*;

#[gpui::test]
pub(crate) async fn a_failed_write_is_not_retried_until_the_next_mutation(cx: &TestAppContext) {
    let _guard = debounce_test_lock();
    let dir = tempdir().unwrap();
    // A regular file where the state file's parent dir would be makes every
    // write fail at ensure_parent_dir.
    let blocker = dir.path().join("blocker");
    std::fs::write(&blocker, b"not a dir").unwrap();
    let state_file = blocker.join("state.json");
    cx.update(|cx| cx.set_global(clean_state(&state_file)));

    cx.update(|cx| update_config(cx, |config| config.sidebar_collapsed = true));
    cx.executor()
        .advance_clock(std::time::Duration::from_millis(DEBOUNCE_MS));
    cx.executor().run_until_parked();
    assert!(
        cx.read(|cx| cx.global::<AppState>().last_save_error.is_some()),
        "the blocked write must record an error"
    );

    // Unblock and wait far past another window: no retry without a mutation.
    std::fs::remove_file(&blocker).unwrap();
    std::fs::create_dir_all(&blocker).unwrap();
    cx.executor()
        .advance_clock(std::time::Duration::from_millis(DEBOUNCE_MS * 10));
    cx.executor().run_until_parked();
    assert!(
        !state_file.exists(),
        "a failed write must not retry on its own"
    );

    cx.update(|cx| {
        update_config(cx, |config| {
            config.theme = crate::app::theme::ThemeMode::Dark
        })
    });
    cx.executor()
        .advance_clock(std::time::Duration::from_millis(DEBOUNCE_MS));
    cx.executor().run_until_parked();

    let loaded = read_persisted(&state_file);
    assert!(loaded.sidebar_collapsed, "the failed payload must retry");
    assert_eq!(loaded.theme, crate::app::theme::ThemeMode::Dark);
}
