# Tabs

`Tabs` owns roving keyboard focus and the moving selection surface. The caller
owns the selected index, tab icons, and panel content.

```rust,ignore
Tabs::new("settings-tabs")
    .label("Settings")
    .items([Tab::new("General"), Tab::new("Developer")])
    .selected_index(selected)
    .on_change(cx.listener(|this, index, _, cx| {
        this.selected = *index;
        cx.notify();
    }))
```

Left/Right wrap, Home/End jump, and Enter/Space activate. Only the selected tab
participates in Tab navigation. `.autofocus(true)` requests focus on that tab.
Use distinct list IDs to keep focus and motion independent.

The selection uses [GPUI Kit motion](https://gpui-kit.com/base/motion/), with a
320 ms critically damped spring. Retargeting preserves position and velocity;
the operating system's reduced-motion preference snaps to the new selection.

The surface uses the Clear material fallback described by
[Bezel Materials](https://bezel.gallery/docs/material.md). Bezel's refractive
lens requires its GPUI fork's `paint_backdrop_blur` and `GlassEffect` APIs.
The installed `gpui-pre 0.3.8` renderer lacks them, so the surface uses Bezel's
translucent tint and rim. Whole-window native blur is configured by the host.

Render all four settings tabs in light and dark with native text shaping and
Metal using `cargo run -p corpora_shell --example settings_preview`.
