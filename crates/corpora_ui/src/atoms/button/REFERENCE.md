# Button reference

Source: Corpora UI commit `e800b8489a287da0120c2c1842deb9748d499b80` in the sibling `corpora-ui` checkout.
The live `/atoms/button` page and supplied Button documentation were reviewed
on 2026-10-22.

- `react/src/components/ui/button.tsx`: variants, sizes, loading, and activation.
- `react/src/registry/demos/button-demo.tsx`: playground controls.
- `react/src/lib/glass-variants.ts`: finish tints and optics.
- `react/src/assets/css/theme.css`: radius and transition easing.

The native component uses the application's semantic palette. It preserves
the 640 px responsive breakpoint in rem units, 150 ms surface transitions,
300 ms size and loading transitions, and reduced motion. Loading blocks
activation and exposes disabled and busy accessibility metadata. Icon sizes
overlay the spinner; text sizes retain the label and animate an inline slot.

Native adaptations:

- Glass uses the source tints with edge lighting. This GPUI version exposes
  neither backdrop blur nor live refraction.
- Press feedback contracts the painted surface and icon while keeping layout
  and hitboxes stable; arbitrary child subtrees have no paint scale API.
- Native callbacks and optional browser URLs replace React's `render` prop.
- Desktop pointer targets use the visible button bounds. The web's coarse
  pointer hit-area expansion is not exposed by this native control.
- The existing application preference suppresses accent focus outlines;
  focus traversal and Enter/Space activation remain available.
- Sound is omitted at the user's request.

Showcase: `/ui/button`, retained independently in each workspace tab.
