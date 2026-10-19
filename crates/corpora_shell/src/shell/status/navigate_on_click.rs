#[allow(unused_imports)]
use super::*;

pub(crate) fn navigate_on_click(item: StatusBarItem, path: &'static str) -> StatusBarItem {
    item.on_click(move |_, window, cx| gpui_router::use_navigate(window, cx).push(path))
}
