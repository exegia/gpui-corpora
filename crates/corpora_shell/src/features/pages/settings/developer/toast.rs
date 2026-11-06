#[allow(unused_imports)]
use super::*;

pub(crate) fn toast(message: String, cx: &mut App) {
    crate::ui::toasts::push(Toast::new(message), cx);
}
