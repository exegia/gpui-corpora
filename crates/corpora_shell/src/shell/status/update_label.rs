#[allow(unused_imports)]
use super::*;

pub(crate) fn update_label(cx: &App) -> Option<String> {
    match &cx.try_global::<updater::UpdateSnapshot>()?.status {
        updater::UpdateStatus::Available { version, .. } => {
            Some(format!("Update {version} available"))
        }
        updater::UpdateStatus::Downloading { progress } => {
            Some(format!("Downloading update {progress}%"))
        }
        updater::UpdateStatus::Downloaded { version, .. } => {
            Some(format!("Update {version} ready"))
        }
        _ => None,
    }
}
