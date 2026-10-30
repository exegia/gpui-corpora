#[allow(unused_imports)]
use super::*;

/// Make `toaster` the target of [`push`] and [`replace`]; the workspace
/// renders it through one `ToastViewport`.
pub fn install(toaster: Entity<Toaster>, cx: &mut App) {
    cx.set_global(AppToaster {
        toaster,
        keyed: HashMap::new(),
    });
}
