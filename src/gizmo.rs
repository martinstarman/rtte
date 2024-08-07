use bevy::{
  color::palettes::css::{BLUE, GREEN},
  prelude::*,
};

pub fn gizmo(mut gizmos: Gizmos) {
  let v = Vec2::ZERO;

  gizmos.arrow_2d(Vec2::new(v.x, v.y), Vec2::new(v.x + 30., v.y), GREEN);
  gizmos.arrow_2d(Vec2::new(v.x, v.y), Vec2::new(v.x, v.y + 30.), BLUE);
}
