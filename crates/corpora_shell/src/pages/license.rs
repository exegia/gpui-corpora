#[allow(unused_imports)]
use super::*;

pub fn license(cx: &App) -> impl IntoElement + use<> {
    page(
        "License details",
        format!("License · {}", parameter(cx, "licenseId")),
    )
    .child(empty(
        "license-detail-empty",
        IconName::Shield,
        "License not loaded",
        "This route is ready for a connected license store.",
    ))
    .child(nav_button(
        "all-licenses",
        "All licenses",
        "/projects/licenses",
    ))
}
