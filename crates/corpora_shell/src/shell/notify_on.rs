#[allow(unused_imports)]
use super::*;

/// Observe global `T` and notify the workspace on change.
pub(crate) fn notify_on<T: Global>(cx: &mut Context<Workspace>) -> Subscription {
    cx.observe_global::<T>(|_, cx| cx.notify())
}
