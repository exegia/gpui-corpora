#[allow(unused_imports)]
use super::*;

#[gpui::test]
pub(crate) async fn an_unchanged_update_skips_the_write_entirely(cx: &TestAppContext) {
    let _guard = debounce_test_lock();
    let dir = tempdir().unwrap();
    let state_file = dir.path().join("state.json");
    cx.update(|cx| cx.set_global(clean_state(&state_file)));

    cx.update(|cx| update_config(cx, |_config| {}));
    cx.executor()
        .advance_clock(std::time::Duration::from_millis(DEBOUNCE_MS));
    cx.executor().run_until_parked();

    assert!(
        !state_file.exists(),
        "a state identical to the last flush must never hit the disk"
    );
    assert!(!cx.read(|cx| cx.global::<AppState>().dirty));
}
