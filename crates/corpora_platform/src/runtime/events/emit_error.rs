#[allow(unused_imports)]
use super::*;

pub fn emit_error(error: AppError, cx: &mut App) {
    let severity = error.severity();
    let message = error.to_string();
    tracing::warn!(
        target: "corpora::events",
        severity = ?severity,
        error = %message,
        "emitting app error"
    );
    emit(AppEventKind::AppError { message, severity }, cx);
}
