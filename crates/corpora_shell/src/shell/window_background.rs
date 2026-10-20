use gpui::{Div, Hsla, div, hsla, linear_color_stop, linear_gradient, prelude::*};

// Each tuple is CSS hue, saturation, lightness, and stop percentage.
const DARK: &[(f32, f32, f32, f32)] = &[
    (0.0, 0.0, 10.0, 0.0),
    (344.0, 0.0, 9.0, 20.0),
    (344.0, 0.0, 8.0, 38.0),
    (344.0, 0.0, 7.0, 53.0),
    (344.0, 0.0, 6.0, 67.0),
    (344.0, 0.0, 5.0, 81.0),
    (0.0, 0.0, 4.0, 100.0),
];

const LIGHT: &[(f32, f32, f32, f32)] = &[
    (0.0, 0.0, 92.0, 0.0),
    (344.0, 0.0, 91.0, 7.0),
    (344.0, 0.0, 91.0, 14.0),
    (344.0, 0.0, 90.0, 20.0),
    (344.0, 0.0, 89.0, 27.0),
    (344.0, 0.0, 88.0, 32.0),
    // (344.0, 0.0, 87.0, 38.0),
    // (344.0, 0.0, 86.0, 43.0),
    // (344.0, 0.0, 86.0, 49.0),
    // (344.0, 0.0, 85.0, 53.0),
    // (344.0, 0.0, 84.0, 58.0),
    // (344.0, 0.0, 83.0, 63.0),
    // (344.0, 0.0, 82.0, 67.0),
    // (344.0, 0.0, 82.0, 72.0),
    // (344.0, 0.0, 81.0, 76.0),
    (344.0, 0.0, 80.0, 81.0),
    (344.0, 0.0, 79.0, 87.0),
    (344.0, 0.0, 78.0, 93.0),
    (0.0, 0.0, 78.0, 100.0),
];

fn color(stop: (f32, f32, f32, f32)) -> Hsla {
    hsla(stop.0 / 360.0, stop.1 / 100.0, stop.2 / 100.0, 1.0)
}

pub(super) fn window_background(dark: bool) -> Div {
    let stops = if dark { DARK } else { LIGHT };
    // GPUI supports two stops per gradient. Successive fades preserve each interval.
    div()
        .absolute()
        .inset_0()
        .bg(color(stops[0]))
        .children(stops.windows(2).map(|pair| {
            let next = color(pair[1]);
            div().absolute().inset_0().bg(linear_gradient(
                0.0,
                linear_color_stop(next.opacity(0.0), pair[0].3 / 100.0),
                linear_color_stop(next, pair[1].3 / 100.0),
            ))
        }))
}
