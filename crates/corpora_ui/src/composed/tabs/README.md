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

On macOS, the surface renders [Bezel Clear glass](https://bezel.gallery/docs/material.md)
with framebuffer sampling, GPU Gaussian blur, rim refraction, and chromatic
dispersion. The renderer backport and source revisions are documented in
[`vendor/README.md`](../../../../../vendor/README.md). The glass stays below the
tab icons and labels; GPUI Kit motion still drives its bounds. Other renderers
retain a translucent fill until they implement the backdrop primitive.

Render all four settings tabs in light and dark with native text shaping and
Metal using `cargo run -p corpora_shell --example settings_preview`.
Verify backdrop displacement, rounded clipping, and the moving glass against a
striped backdrop with `cargo run -p corpora_ui --example tabs_glass_preview`.
