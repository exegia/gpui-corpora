#[allow(unused_imports)]
use super::*;

pub fn search() -> impl IntoElement {
    page("Explore", "Explore the texts in your workspace.")
        .h_full()
        .child(
            div()
                .flex_1()
                .min_h_0()
                .flex()
                .items_center()
                .justify_center()
                .child(empty(
                    "search-empty",
                    IconName::Search,
                    "No texts connected",
                    "Connect a corpus source to search its contents.",
                )),
        )
}
