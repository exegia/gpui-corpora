#[allow(unused_imports)]
use super::*;

/// Show `toast`; dropped with a debug log before the workspace exists.
pub fn push(toast: Toast, cx: &mut App) {
    let Some(toaster) = cx.try_global::<AppToaster>().map(|t| t.toaster.clone()) else {
        tracing::debug!(target: "corpora::toasts", "no toaster installed; toast dropped");
        return;
    };
    toaster.update(cx, |toaster, cx| {
        toaster.push(toast, cx);
    });
}
