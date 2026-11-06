#[allow(unused_imports)]
use super::*;

pub(crate) fn reset_download_retry(cx: &mut gpui::App) {
    UpdateSnapshot::update_global(cx, |snap, _cx| {
        snap.download_retry_count = 0;
    });
}
