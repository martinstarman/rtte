use crate::geometry::{line::Line, vec2::Vec2};

use maths_rs::{line_segment_vs_line_segment, point_inside_triangle, Vec2f};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, PartialEq, Clone, Copy, Debug)]
pub enum Kind {
  #[default]
  GROUND = 0,
  WATER = 1,
  SNOW = 2,
  BLOCK = 3,       // path and view block
  TRANSPARENT = 4, // path block
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct Triangle {
  pub a: Vec2,
  pub b: Vec2,
  pub c: Vec2,
  pub kind: Kind,
  #[serde(skip)]
  pub is_selected: bool,
}

impl Triangle {
  pub fn new(a: Vec2, b: Vec2, c: Vec2, kind: Kind) -> Self {
    Triangle {
      a,
      b,
      c,
      kind,
      is_selected: false,
    }
  }

  pub fn is_blocking_path(&self) -> bool {
    self.kind == Kind::BLOCK || self.kind == Kind::TRANSPARENT
  }

  pub fn is_blocking_view(&self) -> bool {
    self.kind == Kind::BLOCK
  }

  // triangle contains vec2
  pub fn contains(&self, v: Vec2) -> bool {
    point_inside_triangle::<f32, Vec2f>(v.into(), self.a.into(), self.b.into(), self.c.into())
  }

  // line segment intersect triangle
  pub fn intersected(&self, l: Line) -> bool {
    line_segment_vs_line_segment(l.a.into(), l.b.into(), self.a.into(), self.b.into()).is_some()
      || line_segment_vs_line_segment(l.a.into(), l.b.into(), self.b.into(), self.c.into())
        .is_some()
      || line_segment_vs_line_segment(l.a.into(), l.b.into(), self.c.into(), self.a.into())
        .is_some()
  }
}
