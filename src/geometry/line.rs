use crate::geometry::vec2::Vec2;

#[derive(Clone, Copy)]
pub struct Line {
  pub a: Vec2,
  pub b: Vec2,
}

impl Line {
  pub fn new(a: Vec2, b: Vec2) -> Self {
    Line { a, b }
  }
}
