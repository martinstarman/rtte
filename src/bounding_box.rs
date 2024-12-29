use bevy::{
  math::bounding::{Aabb2d, BoundingVolume},
  prelude::*,
};

#[derive(Component)]
pub struct BoundingBox {
  pub value: Aabb2d,
}

pub fn bounding_box_draw(
  query: Query<(&Parent, &BoundingBox)>,
  query2: Query<&Transform>,
  mut gizmos: Gizmos,
) {
  for (parent, bounding_box) in &query {
    if let Ok(transform) = query2.get(**parent) {
      let half_size = bounding_box.value.half_size();
      let rectangle = Rectangle { half_size };

      // TODO: stop using gizmos
      gizmos.primitive_2d(
        &rectangle,
        Isometry2d::from_translation(bounding_box.value.center() + transform.translation.xy()),
        Color::srgb(0., 1., 0.),
      );
    }
  }
}
