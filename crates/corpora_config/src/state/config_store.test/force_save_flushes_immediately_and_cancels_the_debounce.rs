#[allow(unused_imports)]
use super::*;

#[gpui::test]
pub(crate) async fn force_save_flushes_immediately_and_cancels_the_debounce(cx: &TestAppContext) {
    let _guard = debounce_test_lock();
    let dir = tempdir().unwrap();
    let state_file = dir.path().join("state.json");
    cx.update(|cx| cx.set_global(clean_state(&state_file)));

    cx.update(|cx| {
        update_config(cx, |config| {
            config.theme = crate::app::theme::ThemeMode::Dark
        })
    });
    cx.update(force_save);

    let loaded = read_persisted(&state_file);
    assert_eq!(loaded.theme, crate::app::theme::ThemeMode::Dark);
    let task_cancelled = cx.read(|cx| {
        cx.try_global::<AppState>()
            .unwrap()
            .in_flight_save
            .is_none()
    });
    assert!(
        task_cancelled,
        "the debounce task must be cancelled, not left to fire after shutdown"
    );
    assert!(!SAVE_SCHEDULED.load(Ordering::Relaxed));
}
