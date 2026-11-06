#[allow(unused_imports)]
use super::*;

#[gpui::test]
pub(crate) async fn update_config_before_initialize_warns_and_does_nothing(cx: &TestAppContext) {
    cx.update(|cx| update_config(cx, |config| config.sidebar_collapsed = true));
    assert!(
        cx.read(|cx| cx.try_global::<AppState>().is_none()),
        "a missing store must not be created by an update"
    );
    cx.update(force_save);
    assert!(cx.read(|cx| cx.try_global::<AppState>().is_none()));
}
