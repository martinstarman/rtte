mod camera;
mod gizmo;
mod player;

use bevy::{
  dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin},
  prelude::*,
  window::WindowResolution,
};
use camera::{camera_pan, camera_setup};
use gizmo::gizmo;
use player::{player_animation, player_setup};

fn main() -> AppExit {
  App::new()
    .add_plugins((
      DefaultPlugins
        .set(WindowPlugin {
          primary_window: Some(Window {
            title: "RTTE".into(),
            resolution: WindowResolution::new(800., 600.),
            ..Default::default()
          }),
          ..Default::default()
        })
        .set(ImagePlugin::default_nearest()),
      FpsOverlayPlugin {
        config: FpsOverlayConfig {
          text_config: TextStyle {
            font_size: 20.,
            ..default()
          },
        },
      },
    ))
    .add_systems(Startup, (camera_setup, player_setup))
    .add_systems(Update, (camera_pan, gizmo, player_animation))
    .run()
}
