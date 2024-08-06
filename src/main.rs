mod camera;

use bevy::{prelude::*, window::WindowResolution};
use camera::camera_setup;

fn main() -> AppExit {
  App::new()
    .add_plugins(DefaultPlugins.set(WindowPlugin {
      primary_window: Some(Window {
        title: "RTTE".into(),
        resolution: WindowResolution::new(800., 600.),
        ..Default::default()
      }),
      ..Default::default()
    }))
    .add_systems(Startup, (camera_setup, red_square_setup))
    .run()
}

fn red_square_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
  let red_square = SpriteBundle {
    texture: asset_server.load("red_square.png"),
    ..default()
  };

  commands.spawn(red_square);
}
