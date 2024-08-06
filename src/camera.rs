use bevy::prelude::*;

#[derive(Component)]
pub struct Camera;

pub fn camera_setup(mut commands: Commands) {
  let camera2d = Camera2dBundle::default();

  commands.spawn((camera2d, Camera));
}
