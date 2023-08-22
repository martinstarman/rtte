use ggez::mint::Point2;
use maths_rs::Vec2f;

#[derive(Copy, Clone, Default)]
pub struct Vec2 {
  pub x: f32,
  pub y: f32,
}

impl Vec2 {
  pub fn new(x: f32, y: f32) -> Self {
    Vec2 { x, y }
  }
}

impl Into<Point2<f32>> for Vec2 {
  fn into(self) -> Point2<f32> {
    Point2 {
      x: self.x,
      y: self.y,
    }
  }
}

impl Into<Vec2f> for Vec2 {
  fn into(self) -> Vec2f {
    Vec2f {
      x: self.x,
      y: self.y,
    }
  }
}
