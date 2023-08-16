use ggez::mint::{Point2, Vector2};
use maths_rs::{Vec2f, Vec3f};
use std::ops::{Add, Div, Sub};

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

impl Add for Vec2 {
  type Output = Self;

  fn add(self, other: Self) -> Self::Output {
    Self {
      x: self.x + other.x,
      y: self.y + other.y,
    }
  }
}

impl Sub for Vec2 {
  type Output = Self;

  fn sub(self, other: Self) -> Self::Output {
    Self {
      x: self.x - other.x,
      y: self.y - other.y,
    }
  }
}

impl Div<f32> for Vec2 {
  type Output = Self;

  fn div(self, rhs: f32) -> Self::Output {
    Self {
      x: self.x / rhs,
      y: self.y / rhs,
    }
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

impl Into<Vector2<f32>> for Vec2 {
  fn into(self) -> Vector2<f32> {
    Vector2 {
      x: self.x,
      y: self.y,
    }
  }
}

impl Into<Point2<i32>> for Vec2 {
  fn into(self) -> Point2<i32> {
    Point2 {
      x: self.x as i32,
      y: self.y as i32,
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

impl Into<Vec3f> for Vec2 {
  fn into(self) -> Vec3f {
    Vec3f {
      x: self.x,
      y: self.y,
      z: 0.,
    }
  }
}

impl From<Point2<i32>> for Vec2 {
  fn from(p: Point2<i32>) -> Vec2 {
    Self {
      x: p.x as f32,
      y: p.y as f32,
    }
  }
}
