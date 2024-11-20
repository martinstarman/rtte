use bevy::{
  math::bounding::{Aabb2d, BoundingVolume},
  prelude::*,
};

#[derive(Component)]
pub struct BoundingBox {
  pub value: Aabb2d,
}

pub fn bounding_box_draw(query: Query<&BoundingBox>, mut gizmos: Gizmos) {
  for bounding_box in &query {
    let half_size = bounding_box.value.half_size();
    let rectangle = Rectangle { half_size };

    // TODO: stop using gizmos
    gizmos.primitive_2d(
      &rectangle,
      bounding_box.value.center(),
      0.,
      Color::srgb(0., 1., 0.),
    );
  }
}

pub fn bounding_box_translation(
  mut query: Query<(&mut BoundingBox, &Transform), Changed<Transform>>,
) {
  for (mut bounding_box, transform) in &mut query {
    let center = bounding_box.value.center();
    let translation = transform.translation.xy();
    let step = translation - center;

    bounding_box.value.translate_by(step);
  }
}
