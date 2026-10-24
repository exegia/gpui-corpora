#[allow(unused_imports)]
use super::*;

pub fn projects() -> impl IntoElement {
    page("Projects", "Organize your corpus work.")
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
        .child(empty(
            "projects-empty",
            IconName::Folder,
            "No projects yet",
            "Your projects will appear here when project storage is connected.",
        ))
}
