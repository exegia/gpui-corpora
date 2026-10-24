#[allow(unused_imports)]
use super::*;

pub fn not_found() -> impl IntoElement {
    empty(
        "not-found",
        IconName::Search,
        "Page not found",
        "This address does not match a workspace page.",
    )
    .action(nav_button("return-home", "Go home", "/"))
}
