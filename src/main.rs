mod bounding_box;
mod building;
mod camera;
mod direction;
mod gizmo;
mod movable;
mod player;
mod utils;
mod ysort;

use bevy::{
  dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin},
  prelude::*,
  window::WindowResolution,
};
use bounding_box::draw_bounding_box;
use building::building_setup;
use camera::{camera_pan, camera_setup};
use gizmo::gizmo;
use movable::draw_path;
use player::{
  player_animation, player_atlas_layout, player_direction, player_follow_path, player_path,
  player_setup, player_state,
};
use ysort::y_sort;

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
    .add_systems(Startup, (camera_setup, player_setup, building_setup))
    .add_systems(
      Update,
      (
        camera_pan,
        gizmo,
        player_animation,
        player_direction,
        player_path,
        player_follow_path,
        player_state,
        player_atlas_layout,
        draw_bounding_box,
        draw_path,
      ),
    )
    .add_systems(PostUpdate, y_sort)
    .run()
}
