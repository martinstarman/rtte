use bevy::prelude::*;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct YSort {
  pub height: u32,
}

pub fn y_sort(mut query: Query<(&mut Transform, &YSort)>) {
  for (mut transform, ysort) in &mut query {
    transform.translation.z = -transform.translation.y + (ysort.height as f32 / 2.);
  }
}
