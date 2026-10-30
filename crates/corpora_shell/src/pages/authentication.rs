#[allow(unused_imports)]
use super::*;

pub fn authentication() -> impl IntoElement {
    page("Authentication", "Manage access to your workspace.")
        .child(nav_button(
            "profile-information",
            "Information",
            "/profile/information",
        ))
        .child(empty(
            "authentication-empty",
            IconName::Shield,
            "Authentication not configured",
            "Connect an authentication provider to manage sign-in and sessions.",
        ))
}
