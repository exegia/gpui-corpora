#[allow(unused_imports)]
use super::*;

pub(crate) fn command_menu_items(command_ids: &[CommandId], cx: &App) -> Vec<MenuItem> {
    let specs = commands::registry();
    command_ids
        .iter()
        .filter_map(|id| {
            specs.iter().find(|spec| spec.id == *id).map(|spec| {
                let availability = commands::availability(*id, cx);
                let title = if let Some(reason) = availability.disabled_reason {
                    format!("{} ({})", spec.title, reason)
                } else {
                    spec.title.to_string()
                };
                MenuItem::action(title, ExecuteCommand(*id)).disabled(!availability.enabled)
            })
        })
        .collect()
}
