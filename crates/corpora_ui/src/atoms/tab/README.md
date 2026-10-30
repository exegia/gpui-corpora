# Tabs

`Tabs` owns roving keyboard focus and its glass selection surfaces. The caller
owns the selected index, tab content, and panel content.

```rust,ignore
Tabs::new("settings-tabs")
    .label("Settings")
    .variant(TabVariant::Vertical)
    .items([Tab::new("General"), Tab::new("Developer")])
    .selected_index(selected)
    .on_change(cx.listener(|this, index, _, cx| {
        this.selected = *index;
        cx.notify();
    }))
```

Use `TabVariant::Vertical` for icon-over-label navigation such as Settings. It
keeps fixed tab dimensions and moves one shared glass surface with a 320 ms
critically damped spring. This is the default variant.

Use `TabVariant::Horizontal` for workspace tabs. Labels render in full beside a
leading icon, each selected tab owns its fading glass surface, and
`Tab::closable(true)` enables its close button. Handle close requests by index:

```rust,ignore
Tabs::new("workspace-tabs")
    .variant(TabVariant::Horizontal)
    .items([
        Tab::new("UI").icon(ui_icon),
        Tab::new("New tab 1").icon(tab_icon).closable(true),
    ])
    .selected_index(selected)
    .on_change(|index, _, cx| select(*index, cx))
    .on_close(|index, _, cx| close(*index, cx))
```

Both variants use the workspace tab color states: selected foreground,
60%-opacity foreground at rest, semantic hover and pressed fills, and the accent
focus ring. Left/Right wrap, Home/End jump, and Enter/Space activate. Only the
selected tab participates in roving Tab navigation. In the horizontal variant,
the selected close button is the next Tab stop.

Pass `None` to `.selected_index(...)` when the associated panel is not showing;
the first tab remains keyboard-reachable without rendering a selection surface.
`.autofocus(true)` requests focus on the roving tab. Use distinct list IDs to
keep focus and motion independent.

On macOS, both variants render [Bezel Clear glass](https://bezel.gallery/docs/material.md)
with framebuffer sampling, GPU Gaussian blur, rim refraction, and chromatic
dispersion. The renderer backport and source revisions are documented in
[`vendor/README.md`](../../../../../vendor/README.md). Other renderers retain a
translucent fill until they implement the backdrop primitive.

Render all four settings tabs in light and dark with native text shaping and
Metal using `cargo run -p corpora_shell --example settings_preview`. Verify
backdrop displacement and clipping against a striped backdrop with
`cargo run -p corpora_ui --example tabs_glass_preview`.
