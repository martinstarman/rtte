use bevy::{
  math::bounding::{Aabb2d, BoundingVolume},
  prelude::*,
};

#[derive(Component)]
pub struct BoundingBox {
  pub value: Aabb2d,
}

pub fn draw_bounding_box(query: Query<&BoundingBox>, mut gizmos: Gizmos) {
  for bounding_box in &query {
    let half_size = bounding_box.value.half_size();
    let rectandle = Rectangle { half_size };

    // TODO: do not use gizmos
    gizmos.primitive_2d(&rectandle, bounding_box.value.center(), 0., Color::WHITE);
  }
}
