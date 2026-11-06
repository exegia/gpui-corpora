#[allow(unused_imports)]
use super::*;

pub fn explore_detail(cx: &App) -> impl IntoElement + use<> {
    let kind = parameter(cx, "type");
    let id = parameter(cx, "id");
    page("Text details", format!("{kind} · {id}"))
        .child(empty(
            "text-detail-empty",
            IconName::BookOpen,
            "Text not loaded",
            "This route is ready for a connected corpus source.",
        ))
        .child(nav_button(
            "open-viewer",
            "Open viewer",
            format!("/explore/detail/{kind}/{id}/viewer"),
        ))
}
