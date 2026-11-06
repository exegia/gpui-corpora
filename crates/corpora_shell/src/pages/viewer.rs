#[allow(unused_imports)]
use super::*;

pub fn viewer(cx: &App) -> impl IntoElement + use<> {
    let kind = parameter(cx, "type");
    let id = parameter(cx, "id");
    page("Viewer", format!("{kind} · {id}"))
        .child(empty(
            "viewer-empty",
            IconName::BookOpen,
            "Nothing to read yet",
            "Text content will appear here when a source is connected.",
        ))
        .child(nav_button(
            "back-to-text",
            "Text details",
            format!("/explore/detail/{kind}/{id}"),
        ))
}
