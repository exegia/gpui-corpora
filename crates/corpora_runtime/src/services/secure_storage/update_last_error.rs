#[allow(unused_imports)]
use super::*;

pub(crate) fn update_last_error(last_error: Option<String>, cx: &mut App) {
    cx.update_global::<SecureStorageSnapshot, _>(|state, _cx| {
        state.last_error = last_error;
    });
}
