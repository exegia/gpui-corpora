# GPUI Bezel glass backport

These are the published `gpui-pre` and `gpui-pre-apple` 0.3.8 sources, based on
Zed revision `279fe070bb389b79652e52065b2f001edcc0b11b`. Cargo patches both
packages locally so Ely, GPUI Kit, routing, and Agentation share one GPUI type
graph. Their Apache-2.0 licenses are preserved in each package.

The glass implementation is adapted from
[`crabtalk/zed` revision `44df48a`](https://github.com/crabtalk/zed/tree/44df48a262b7bba0f2ca6bceddbf1a5abcfbee8e).
Only these source files differ from the published packages:

- `gpui-pre/src/scene.rs`: ordered backdrop primitives and retained-scene replay.
- `gpui-pre/src/window.rs`: `GlassEffect` and `paint_backdrop_blur`.
- `gpui-pre-apple/src/metal_renderer.rs`: framebuffer snapshots, cached MPS Gaussian
  kernels/textures, and a replacement glass pass with premultiplied alpha.
- `gpui-pre-apple/src/shaders.metal`: Bezel's rounded lens, dispersion, and rim.
- `gpui-pre-apple/build.rs`: generate Metal bindings from the patched GPUI core.

The unused embedded core copy in `gpui-pre-apple/vendor` is omitted. Registry
cache markers and lockfiles are omitted; the application lockfile remains the
dependency lock. The workspace manifest pins these two local packages without
upgrading their dependencies.

Compatibility adjustments use GPUI 0.3.8's element opacity and rectangular
content mask. Metal already permits framebuffer sampling. Its ordinary draw
pipelines now accumulate alpha with source-over, as Bezel's glass shader needs
the framebuffer's premultiplied color and coverage to agree.

The Clear material values come from
[`crabtalk/bezel` revision `556c045`](https://github.com/crabtalk/bezel/blob/556c045372074c5cf76426626f19fb376fdfb2c0/crates/theme/src/theme/palettes.rs).
They are used under its [MIT license](LICENSE-BEZEL). The native Metal path is
implemented; other renderers continue to use the translucent material fallback.
