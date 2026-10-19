#[allow(unused_imports)]
use super::*;

pub fn search() -> impl IntoElement {
    page("Search", "Explore the texts in your workspace.").child(empty(
        "search-empty",
        IconName::Search,
        "No texts connected",
        "Connect a corpus source to search its contents.",
    ))
}
