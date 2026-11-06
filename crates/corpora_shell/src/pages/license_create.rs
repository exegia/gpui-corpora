#[allow(unused_imports)]
use super::*;

pub fn license_create() -> impl IntoElement {
    page(
        "Create license",
        "Define permissions for your project content.",
    )
    .child(empty(
        "license-create-empty",
        IconName::Shield,
        "License storage not connected",
        "License creation will be available after a storage service is connected.",
    ))
    .child(nav_button(
        "cancel-license",
        "All licenses",
        "/projects/licenses",
    ))
}
