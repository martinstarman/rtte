use bevy::{camera_controller::pan_camera::PanCamera, prelude::*};

#[derive(Component)]
pub struct MainCamera;

pub fn camera_init(mut commands: Commands) {
  let camera2d = Camera2d::default();
  commands.spawn((camera2d, MainCamera, PanCamera::default()));
}
