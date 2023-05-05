use crate::geometry::vec2::Vec2;

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct LineSegment {
  pub a: Vec2,
  pub b: Vec2,
}

impl LineSegment {
  pub fn new(a: Vec2, b: Vec2) -> Self {
    LineSegment { a, b }
  }
}
