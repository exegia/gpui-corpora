#[allow(unused_imports)]
use super::*;

pub(crate) fn push_in_app_feedback(message: impl Into<SharedString>, cx: &mut App) {
    let message = message.into();
    tracing::debug!(target: LOG, message = %message, "showing in-app notification feedback");
    crate::ui::toasts::push(corpora_ui::ely::feedback::Toast::new(message), cx);
}
