#[allow(unused_imports)]
use super::*;

pub fn nav_button(id: &'static str, label: &'static str, path: impl Into<String>) -> Button {
    let path = path.into();
    Button::new(id, label).on_click(move |_, window, cx| {
        use_navigate(window, cx).push(path.clone());
    })
}
