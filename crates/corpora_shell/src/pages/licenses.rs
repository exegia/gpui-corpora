#[allow(unused_imports)]
use super::*;

pub fn licenses() -> impl IntoElement {
    page("Licenses", "Manage how your project content can be used.")
        .child(
            nav_button(
                "create-license",
                "Create license",
                "/projects/licenses/create",
            )
            .primary(),
        )
        .child(empty(
            "licenses-empty",
            IconName::Shield,
            "No licenses yet",
            "Saved licenses will appear here when storage is connected.",
        ))
}
