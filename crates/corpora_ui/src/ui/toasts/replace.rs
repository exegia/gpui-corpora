#[allow(unused_imports)]
use super::*;

/// Show `toast`, dismissing the last toast pushed under `key`.
pub fn replace(key: impl Into<String>, toast: Toast, cx: &mut App) {
    let Some(state) = cx.try_global::<AppToaster>() else {
        tracing::debug!(target: "corpora::toasts", "no toaster installed; toast dropped");
        return;
    };
    let key = key.into();
    let toaster = state.toaster.clone();
    let previous = state.keyed.get(&key).copied();
    let id = toaster.update(cx, |toaster, cx| {
        if let Some(previous) = previous {
            toaster.dismiss(previous, cx);
        }
        toaster.push(toast, cx)
    });
    cx.global_mut::<AppToaster>().keyed.insert(key, id);
}
