#[allow(unused_imports)]
use super::*;

pub(crate) fn sidebar(path: &str) -> impl IntoElement + use<> {
    let entry = |id, label, to: &'static str| {
        pages::nav_button(id, label, to)
            .full_width()
            .variant(if path == to {
                ButtonVariant::Secondary
            } else {
                ButtonVariant::Ghost
            })
    };
    Sidebar::new("navigation-sidebar", false)
        .child(
            div()
                .p_3()
                .flex()
                .flex_col()
                .gap_2()
                .child(Overline::new("Workspace"))
                .child(entry("nav-home", "Home", navigation::HOME))
                .child(entry("nav-search", "Search", navigation::EXPLORE))
                .child(entry("nav-projects", "Projects", navigation::PROJECTS))
                .child(entry(
                    "nav-create-project",
                    "Create project",
                    "/projects/create",
                ))
                .child(entry("nav-statuses", "Statuses", "/projects/statuses"))
                .child(entry("nav-licenses", "Licenses", "/projects/licenses"))
                .child(entry(
                    "nav-create-license",
                    "Create license",
                    "/projects/licenses/create",
                )),
        )
        .child(
            div()
                .p_3()
                .flex()
                .flex_col()
                .gap_2()
                .child(Overline::new("Account"))
                .child(entry("nav-information", "Information", navigation::PROFILE))
                .child(entry(
                    "nav-authentication",
                    "Authentication",
                    "/profile/authentication",
                )),
        )
        .child(
            div()
                .p_3()
                .flex()
                .flex_col()
                .gap_2()
                .child(Overline::new("App"))
                .child(entry(
                    "nav-notifications",
                    "Notifications",
                    navigation::NOTIFICATIONS,
                ))
                .child(entry(
                    "nav-diagnostics",
                    "Diagnostics",
                    navigation::DIAGNOSTICS,
                ))
                .child(entry("nav-about", "About", navigation::ABOUT)),
        )
        .child(
            div()
                .p_3()
                .child(Caption::new("Your texts, projects, and reading workspace.")),
        )
}
