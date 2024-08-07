use bevy::prelude::*;

#[derive(Component)]
pub struct Camera;

pub fn camera_setup(mut commands: Commands) {
  let camera2d = Camera2dBundle {
    transform: Transform::from_xyz(0., 10., 10.).looking_at(Vec3::ZERO, Vec3::Y),
    ..default()
  };

  commands.spawn((camera2d, Camera));
}
