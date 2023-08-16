use crate::geometry::vec2::Vec2;
use maths_rs::{line_segment_vs_line_segment, point_inside_triangle, Vec2f};

#[derive(PartialEq, Clone, Copy)]
pub enum Kind {
  /// Ground. It does not block anything or leave any marks.
  GROUND = 0,

  /// Any object that blocks enemy view and path finding (house, tree, rock, ...).
  BLOCK = 1,

  /// Any object that blocks only path finding. Like fence.
  TRANSPARENT = 2,

  /// Water.
  WATER = 3,

  /// Snow. Leave marks.
  SNOW = 4,
}

#[derive(Clone, Copy)]
pub struct Triangle {
  pub a: Vec2,
  pub b: Vec2,
  pub c: Vec2,
  pub kind: Kind,
}

impl Triangle {
  pub fn new(a: Vec2, b: Vec2, c: Vec2, kind: Kind) -> Self {
    Triangle { a, b, c, kind }
  }

  pub fn is_blocking_path(&self) -> bool {
    // TODO: water should not block for divers
    self.kind == Kind::BLOCK || self.kind == Kind::TRANSPARENT || self.kind == Kind::WATER
  }

  pub fn is_blocking_view(&self) -> bool {
    self.kind == Kind::BLOCK
  }

  // triangle contains vec2
  pub fn contains(&self, v: Vec2) -> bool {
    point_inside_triangle::<f32, Vec2f>(v.into(), self.a.into(), self.b.into(), self.c.into())
  }

  // line segment intersect triangle
  pub fn intersected(&self, a: Vec2, b: Vec2) -> bool {
    line_segment_vs_line_segment(a.into(), b.into(), self.a.into(), self.b.into()).is_some()
      || line_segment_vs_line_segment(a.into(), b.into(), self.b.into(), self.c.into()).is_some()
      || line_segment_vs_line_segment(a.into(), b.into(), self.c.into(), self.a.into()).is_some()
  }
}
