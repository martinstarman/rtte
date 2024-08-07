use bevy::prelude::*;

const PAN_SPEED: f32 = 5.;

#[derive(Component)]
pub struct Camera;

pub fn camera_setup(mut commands: Commands) {
  let camera2d = Camera2dBundle {
    transform: Transform::from_xyz(0., 10., 10.).looking_at(Vec3::ZERO, Vec3::Y),
    ..default()
  };

  commands.spawn((camera2d, Camera));
}

pub fn camera_pan(
  mut transform_q: Query<&mut Transform, With<Camera>>,
  keys_r: Res<ButtonInput<KeyCode>>,
) {
  let mut transform = transform_q.single_mut();

  if keys_r.pressed(KeyCode::ArrowLeft) {
    transform.translation.x -= PAN_SPEED;
  }

  if keys_r.pressed(KeyCode::ArrowRight) {
    transform.translation.x += PAN_SPEED;
  }

  if keys_r.pressed(KeyCode::ArrowUp) {
    transform.translation.y += PAN_SPEED;
  }

  if keys_r.pressed(KeyCode::ArrowDown) {
    transform.translation.y -= PAN_SPEED;
  }
}
