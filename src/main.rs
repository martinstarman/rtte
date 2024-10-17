mod bounding_box;
mod camera;
mod direction;
mod gizmo;
mod movable;
mod navmesh;
mod player;
mod tree;
mod utils;
mod ysort;

use bevy::{
  dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin},
  prelude::*,
  window::WindowResolution,
};
use bounding_box::draw_bounding_box;
use camera::{camera_pan, camera_setup};
use gizmo::gizmo;
use movable::draw_path;
use navmesh::{navmesh_draw, navmesh_obstacle_draw, navmesh_setup};
use player::{
  player_animation, player_atlas_layout, player_direction, player_follow_path, player_path,
  player_setup, player_state,
};
use tree::tree_setup;
use vleue_navigator::{
  prelude::{NavmeshUpdaterPlugin, PrimitiveObstacle},
  VleueNavigatorPlugin,
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
      VleueNavigatorPlugin,
      NavmeshUpdaterPlugin::<PrimitiveObstacle>::default(),
    ))
    .add_systems(
      Startup,
      (camera_setup, player_setup, tree_setup, navmesh_setup),
    )
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
        draw_bounding_box, // TODO: bounding_box_draw
        draw_path,         // TODO: path_draw
        navmesh_draw,
        navmesh_obstacle_draw,
      ),
    )
    .add_systems(PostUpdate, y_sort)
    .run()
}
