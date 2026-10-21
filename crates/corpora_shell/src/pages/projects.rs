#[allow(unused_imports)]
use super::*;

pub fn projects() -> impl IntoElement {
    page("Projects", "Organize your corpus work.")
        .h_full()
        .child(
            div()
                .flex()
                .gap_2()
                .child(nav_button("create-project", "Create project", "/projects/create").primary())
                .child(nav_button(
                    "project-statuses",
                    "Statuses",
                    "/projects/statuses",
                ))
                .child(nav_button(
                    "project-licenses",
                    "Licenses",
                    "/projects/licenses",
                )),
        )
        .child(
            div()
                .flex_1()
                .min_h_0()
                .flex()
                .items_center()
                .justify_center()
                .child(empty(
                    "projects-empty",
                    IconName::Folder,
                    "No projects yet",
                    "Your projects will appear here when project storage is connected.",
                )),
        )
}
