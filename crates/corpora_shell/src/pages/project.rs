#[allow(unused_imports)]
use super::*;

pub fn project(
    workspace: WeakEntity<Workspace>,
    creating: bool,
    cx: &App,
) -> impl IntoElement + use<> {
    let id = if creating {
        "draft".to_owned()
    } else {
        parameter(cx, "projectId")
    };
    let status = workspace
        .upgrade()
        .and_then(|entity| entity.read(cx).project_statuses.get(&id).copied())
        .unwrap_or(Status::Backlog);
    page(if creating { "Create project" } else { "Project details" }, if creating {
        "Preview a new project's workflow. Storage is not connected.".to_owned()
    } else {
        format!("Project · {id}")
    })
        .child(Card::new()
            .header(CardHeader::new("Workflow"))
            .child(PriorityIndicator::new("project-priority", Priority::None).labeled())
            .child(StatusSelect::new("project-status", status).on_change(move |status, _, cx| {
                workspace.update(cx, |state, cx| {
                    state.project_statuses.insert(id.clone(), status);
                    cx.notify();
                }).ok();
            }))
            .child(Caption::new("Status changes are kept for this session only.")))
        .child(empty("project-detail-empty", IconName::Folder, "Project storage not connected", "Project creation and saved details will be available after a storage service is connected."))
        .child(nav_button("back-to-projects", "All projects", "/projects"))
}
