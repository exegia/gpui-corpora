#[allow(unused_imports)]
use super::*;

pub(crate) fn report_error(
    message: impl Into<String>,
    category: crate::error_surface::ErrorCategory,
    actions: Vec<crate::error_surface::ErrorAction>,
    cx: &mut App,
) {
    crate::error_surface::report(
        message,
        crate::errors::AppErrorSeverity::Error,
        category,
        actions,
        cx,
    );
}
