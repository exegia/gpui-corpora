#[allow(unused_imports)]
use super::*;

pub(crate) fn reset_check_retry(cx: &mut gpui::App) {
    UpdateSnapshot::update_global(cx, |snap, _cx| {
        snap.check_retry_count = 0;
    });
}
