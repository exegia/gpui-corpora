#[allow(unused_imports)]
use super::*;

pub(crate) fn navigate(path: &str, cx: &mut App) {
    events::emit(AppEventKind::Navigate(path.to_string()), cx);
}
