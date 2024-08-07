mod camera;
mod gizmo;

use bevy::{
  dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin},
  prelude::*,
  window::WindowResolution,
};
use camera::{camera_pan, camera_setup};
use gizmo::gizmo;

fn main() -> AppExit {
  App::new()
    .add_plugins((
      DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
          title: "RTTE".into(),
          resolution: WindowResolution::new(800., 600.),
          ..Default::default()
        }),
        ..Default::default()
      }),
      FpsOverlayPlugin {
        config: FpsOverlayConfig {
          text_config: TextStyle {
            font_size: 20.,
            ..default()
          },
        },
      },
    ))
    .add_systems(Startup, (camera_setup, red_square_setup))
    .add_systems(Update, (camera_pan, gizmo))
    .run()
}

fn red_square_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
  let red_square = SpriteBundle {
    texture: asset_server.load("red_square.png"),
    ..default()
  };

  commands.spawn(red_square);
}
