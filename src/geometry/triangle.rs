use crate::geometry::{line::Line, vec2::Vec2};

use maths_rs::{line_segment_vs_line_segment, point_inside_triangle, Vec2f};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct Triangle {
  pub a: Vec2,
  pub b: Vec2,
  pub c: Vec2,
  pub is_path_block: bool,
  pub is_view_block: bool,
  #[serde(skip)]
  pub is_selected: bool,
}

impl Triangle {
  pub fn new(a: Vec2, b: Vec2, c: Vec2) -> Self {
    Triangle {
      a,
      b,
      c,
      is_path_block: false,
      is_view_block: false,
      is_selected: false,
    }
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
