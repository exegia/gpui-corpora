#[allow(unused_imports)]
use super::*;

pub fn information() -> impl IntoElement {
    page("Information", "Your profile and account details.")
        .child(nav_button(
            "profile-authentication",
            "Authentication",
            "/profile/authentication",
        ))
        .child(empty(
            "profile-empty",
            IconName::User,
            "Not signed in",
            "Your profile information will appear when an account is connected.",
        ))
}
