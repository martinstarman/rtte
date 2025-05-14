use bevy::prelude::*;

const PAN_SPEED: f32 = 5.;

#[derive(Component)]
pub struct MainCamera;

pub fn camera_setup(mut commands: Commands) {
  let camera2d = Camera2d::default();
  commands.spawn((camera2d, MainCamera));
}

pub fn camera_pan(
  mut transform: Query<&mut Transform, With<MainCamera>>,
  keys: Res<ButtonInput<KeyCode>>,
) {
  let mut transform = transform.single_mut().unwrap();

  if keys.pressed(KeyCode::ArrowLeft) {
    transform.translation.x -= PAN_SPEED;
  }

  if keys.pressed(KeyCode::ArrowRight) {
    transform.translation.x += PAN_SPEED;
  }

  if keys.pressed(KeyCode::ArrowUp) {
    transform.translation.y += PAN_SPEED;
  }

  if keys.pressed(KeyCode::ArrowDown) {
    transform.translation.y -= PAN_SPEED;
  }
}
