#[allow(unused_imports)]
use super::*;

pub(crate) fn frame_time_label(cx: &App) -> Option<String> {
    if !crate::app_state::with_config(cx, |c| c.show_frame_time) {
        return None;
    }
    let us = super::super::last_frame_time_us();
    let ms = us as f64 / 1000.0;
    let slow = super::super::frame_time::is_slow_frame(us);
    Some(format!(
        "Frame {ms:.2}ms{}",
        if slow { " (slow)" } else { "" }
    ))
}
