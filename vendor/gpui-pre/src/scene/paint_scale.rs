use super::*;

#[derive(Clone, Copy)]
pub(super) struct PaintScale {
    pub center: Point<ScaledPixels>,
    pub factor: f32,
    pub clip: ContentMask<ScaledPixels>,
}

impl PaintScale {
    fn point(self, point: Point<ScaledPixels>) -> Point<ScaledPixels> {
        self.center + (point - self.center) * self.factor
    }

    pub fn bounds(self, bounds: Bounds<ScaledPixels>) -> Bounds<ScaledPixels> {
        Bounds {
            origin: self.point(bounds.origin),
            size: bounds.size.map(|length| length * self.factor),
        }
    }

    fn mask(self, mask: ContentMask<ScaledPixels>) -> ContentMask<ScaledPixels> {
        let transformed = self.bounds(mask.bounds);
        let clip = self.clip.bounds;
        let left = if mask.bounds.left() <= clip.left() {
            clip.left()
        } else {
            transformed.left().max(clip.left())
        };
        let top = if mask.bounds.top() <= clip.top() {
            clip.top()
        } else {
            transformed.top().max(clip.top())
        };
        let right = if mask.bounds.right() >= clip.right() {
            clip.right()
        } else {
            transformed.right().min(clip.right())
        };
        let bottom = if mask.bounds.bottom() >= clip.bottom() {
            clip.bottom()
        } else {
            transformed.bottom().min(clip.bottom())
        };
        ContentMask {
            bounds: Bounds {
                origin: point(left, top),
                size: crate::size(
                    (right - left).max(ScaledPixels(0.)),
                    (bottom - top).max(ScaledPixels(0.)),
                ),
            },
        }
    }

    fn background(self, background: &mut Background) {
        match background.tag {
            crate::BackgroundTag::Checkerboard => {
                background.gradient_angle_or_pattern_height *= self.factor
            }
            crate::BackgroundTag::PatternSlash => {
                let packed = background.gradient_angle_or_pattern_height as u32;
                let width = ((packed / 0xffff) as f32 * self.factor) as u32;
                let interval = ((packed % 0xffff) as f32 * self.factor) as u32;
                background.gradient_angle_or_pattern_height = (width * 0xffff + interval) as f32;
            }
            _ => {}
        }
    }

    fn matrix(self, matrix: &mut TransformationMatrix) {
        let offset = self.center * (1. - self.factor);
        let [x, y] = [offset.x.0, offset.y.0];
        matrix.translation = [
            self.factor * matrix.translation[0] + x
                - matrix.rotation_scale[0][0] * x
                - matrix.rotation_scale[0][1] * y,
            self.factor * matrix.translation[1] + y
                - matrix.rotation_scale[1][0] * x
                - matrix.rotation_scale[1][1] * y,
        ];
    }

    pub fn blur(self, blur: &mut BackdropBlur) {
        blur.bounds = self.bounds(blur.bounds);
        blur.content_mask = self.mask(blur.content_mask);
        blur.corner_radii = blur.corner_radii.map(|radius| *radius * self.factor);
        blur.blur_radius *= self.factor;
        blur.lens *= self.factor;
        blur.reach *= self.factor;
        blur.edge_width *= self.factor;
        blur.edge_aa *= self.factor;
    }

    pub fn primitive(self, primitive: &mut Primitive) {
        match primitive {
            Primitive::Quad(quad) => {
                self.background(&mut quad.background);
                quad.bounds = self.bounds(quad.bounds);
                quad.content_mask = self.mask(quad.content_mask);
                quad.corner_radii = quad.corner_radii.map(|radius| *radius * self.factor);
                quad.border_widths = quad.border_widths.map(|width| *width * self.factor);
            }
            Primitive::Shadow(shadow) => {
                shadow.bounds = self.bounds(shadow.bounds);
                shadow.element_bounds = self.bounds(shadow.element_bounds);
                shadow.content_mask = self.mask(shadow.content_mask);
                shadow.blur_radius *= self.factor;
                shadow.corner_radii = shadow.corner_radii.map(|radius| *radius * self.factor);
                shadow.element_corner_radii = shadow
                    .element_corner_radii
                    .map(|radius| *radius * self.factor);
            }
            Primitive::Underline(underline) => {
                underline.bounds = self.bounds(underline.bounds);
                underline.content_mask = self.mask(underline.content_mask);
                underline.thickness *= self.factor;
            }
            Primitive::MonochromeSprite(sprite) => {
                sprite.bounds = self.bounds(sprite.bounds);
                sprite.content_mask = self.mask(sprite.content_mask);
                self.matrix(&mut sprite.transformation);
            }
            Primitive::SubpixelSprite(sprite) => {
                sprite.bounds = self.bounds(sprite.bounds);
                sprite.content_mask = self.mask(sprite.content_mask);
                self.matrix(&mut sprite.transformation);
            }
            Primitive::PolychromeSprite(sprite) => {
                sprite.bounds = self.bounds(sprite.bounds);
                sprite.content_mask = self.mask(sprite.content_mask);
                sprite.corner_radii = sprite.corner_radii.map(|radius| *radius * self.factor);
            }
            Primitive::Path(path) => {
                self.background(&mut path.color);
                path.bounds = self.bounds(path.bounds);
                path.content_mask = self.mask(path.content_mask);
                path.start = self.point(path.start);
                path.current = self.point(path.current);
                for vertex in &mut path.vertices {
                    vertex.xy_position = self.point(vertex.xy_position);
                    vertex.content_mask = self.mask(vertex.content_mask);
                }
            }
            Primitive::Surface(surface) => {
                surface.bounds = self.bounds(surface.bounds);
                surface.content_mask = self.mask(surface.content_mask);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bounds(x: f32, y: f32, width: f32, height: f32) -> Bounds<ScaledPixels> {
        Bounds {
            origin: point(ScaledPixels(x), ScaledPixels(y)),
            size: crate::size(ScaledPixels(width), ScaledPixels(height)),
        }
    }

    fn quad(rect: Bounds<ScaledPixels>, clip: ContentMask<ScaledPixels>) -> Quad {
        Quad {
            bounds: rect,
            content_mask: clip,
            corner_radii: Corners::all(ScaledPixels(6.)),
            border_widths: Edges::all(ScaledPixels(2.)),
            ..Quad::default()
        }
    }

    #[test]
    fn replay_scales_raw_geometry_once_and_restores_the_scope() {
        let clip = ContentMask {
            bounds: bounds(0., 0., 500., 500.),
        };
        let center = point(ScaledPixels(100.), ScaledPixels(100.));
        let rect = bounds(40., 30., 100., 50.);
        let mut source = Scene::default();
        source.push_scale(center, 0.98, clip);
        source.push_layer(rect);
        let child_start = source.len();
        source.insert_primitive(quad(rect, clip));
        let child_end = source.len();
        source.pop_layer();
        source.pop_scale();
        source.insert_primitive(quad(rect, clip));
        let mut replayed = Scene::default();
        replayed.replay(0..source.len(), &source);
        assert_eq!(replayed.quads[0].bounds, source.quads[0].bounds);
        assert_eq!(replayed.quads[0].corner_radii, source.quads[0].corner_radii);
        assert_eq!(replayed.quads[1].bounds, rect);
        let mut child_replay = Scene::default();
        child_replay.push_scale(center, 0.9, clip);
        child_replay.replay(child_start..child_end, &source);
        child_replay.pop_scale();
        assert_eq!(child_replay.quads[0].bounds.size.width, ScaledPixels(90.));
        assert_eq!(child_replay.quads[0].bounds.origin.x, ScaledPixels(46.));
    }

    #[test]
    fn scaling_keeps_ancestor_clip_fixed_and_can_reveal_cached_geometry() {
        let clip = ContentMask {
            bounds: bounds(0., 0., 500., 500.),
        };
        let center = point(ScaledPixels(100.), ScaledPixels(100.));
        let mut source = Scene::default();
        source.insert_primitive(quad(bounds(-10., 20., 9., 10.), clip));
        assert!(source.quads.is_empty());
        let mut replayed = Scene::default();
        replayed.push_scale(center, 0.98, clip);
        replayed.replay(0..source.len(), &source);
        replayed.pop_scale();
        assert_eq!(replayed.quads.len(), 1);
        assert_eq!(replayed.quads[0].content_mask, clip);
        assert!(replayed.quads[0].bounds.right().0 > 0.);
    }

    #[test]
    fn sprite_transform_composes_with_scale_without_moving_its_rotation_center() {
        let scale = PaintScale {
            center: point(ScaledPixels(100.), ScaledPixels(80.)),
            factor: 0.98,
            clip: ContentMask {
                bounds: bounds(0., 0., 500., 500.),
            },
        };
        let mut matrix = TransformationMatrix {
            rotation_scale: [[0., -1.], [1., 0.]],
            translation: [120., -30.],
        };
        let original = matrix;
        scale.matrix(&mut matrix);
        let apply = |m: TransformationMatrix, p: Point<ScaledPixels>| {
            point(
                ScaledPixels(
                    m.rotation_scale[0][0] * p.x.0
                        + m.rotation_scale[0][1] * p.y.0
                        + m.translation[0],
                ),
                ScaledPixels(
                    m.rotation_scale[1][0] * p.x.0
                        + m.rotation_scale[1][1] * p.y.0
                        + m.translation[1],
                ),
            )
        };
        let point = point(ScaledPixels(30.), ScaledPixels(40.));
        let expected = scale.point(apply(original, point));
        let actual = apply(matrix, scale.point(point));
        assert!((actual.x.0 - expected.x.0).abs() < 0.001);
        assert!((actual.y.0 - expected.y.0).abs() < 0.001);
    }

    #[test]
    fn nested_scale_and_local_clipping_replay_identically() {
        let clip = ContentMask {
            bounds: bounds(0., 0., 500., 500.),
        };
        let local = ContentMask {
            bounds: bounds(45., 40., 80., 30.),
        };
        let center = point(ScaledPixels(100.), ScaledPixels(100.));
        let mut source = Scene::default();
        source.push_scale(center, 0.98, clip);
        source.push_scale(center, 0.9, local);
        source.insert_primitive(quad(bounds(40., 30., 100., 50.), local));
        source.pop_scale();
        source.pop_scale();
        let mut replayed = Scene::default();
        replayed.replay(0..source.len(), &source);
        assert_eq!(source.quads[0].bounds, replayed.quads[0].bounds);
        assert_eq!(source.quads[0].content_mask, replayed.quads[0].content_mask);
        assert!((source.quads[0].bounds.size.width.0 - 88.2).abs() < 0.001);
    }
}
