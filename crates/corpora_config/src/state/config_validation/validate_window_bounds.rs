#[allow(unused_imports)]
use super::*;

pub(crate) fn validate_window_bounds(bounds: Option<&PersistedWindowBounds>) -> Vec<ConfigLint> {
    let Some(b) = bounds else {
        return Vec::new();
    };

    let mut out = Vec::new();
    let dims = [
        ("window_bounds.x", b.x),
        ("window_bounds.y", b.y),
        ("window_bounds.width", b.width),
        ("window_bounds.height", b.height),
    ];

    for (field, value) in dims {
        if value.is_nan() {
            out.push(ConfigLint::new(
                field,
                LintSeverity::Error,
                format!("{field} is NaN; falling back to default window placement"),
            ));
        } else if value.is_infinite() {
            out.push(ConfigLint::new(
                field,
                LintSeverity::Error,
                format!("{field} is infinite; falling back to default window placement"),
            ));
        } else if value < 0.0 {
            // A negative x/y is legal on some multi-monitor setups, but a
            // negative width/height is never meaningful.
            let severity = if field.ends_with("width") || field.ends_with("height") {
                LintSeverity::Error
            } else {
                LintSeverity::Warning
            };
            out.push(ConfigLint::new(
                field,
                severity,
                format!("{field} = {value} is negative; the window may not be visible"),
            ));
        }
    }

    if b.width.is_finite() && (b.width < MIN_WINDOW_DIM || b.width > MAX_WINDOW_DIM) {
        out.push(ConfigLint::new(
            "window_bounds.width",
            LintSeverity::Warning,
            format!(
                "window_bounds.width = {} is outside the expected range [{}, {}]; \
                 it may be clamped by the platform",
                b.width, MIN_WINDOW_DIM, MAX_WINDOW_DIM
            ),
        ));
    }
    if b.height.is_finite() && (b.height < MIN_WINDOW_DIM || b.height > MAX_WINDOW_DIM) {
        out.push(ConfigLint::new(
            "window_bounds.height",
            LintSeverity::Warning,
            format!(
                "window_bounds.height = {} is outside the expected range [{}, {}]; \
                 it may be clamped by the platform",
                b.height, MIN_WINDOW_DIM, MAX_WINDOW_DIM
            ),
        ));
    }

    // Complements the per-dimension ceiling: two legal-but-large dimensions.
    if b.width.is_finite()
        && b.height.is_finite()
        && b.width > 0.0
        && b.height > 0.0
        && (b.width * b.height) > (MAX_WINDOW_DIM * MAX_WINDOW_DIM)
    {
        out.push(ConfigLint::new(
            "window_bounds",
            LintSeverity::Warning,
            format!(
                "window area {}x{} is implausibly large; likely a corrupted value",
                b.width, b.height
            ),
        ));
    }

    out
}
