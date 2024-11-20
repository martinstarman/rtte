mod animation;
mod bounding_box;
mod camera;
mod direction;
mod enemy;
mod gizmo;
mod line_of_sight;
mod movable;
mod navmesh;
mod obstacle;
mod player;
mod tree;
mod utils;
mod ysort;

use bevy::{
  dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin},
  prelude::*,
  window::WindowResolution,
};
use bounding_box::{bounding_box_draw, bounding_box_translation};
use camera::{camera_pan, camera_setup};
use enemy::{enemy_atlas_layout, enemy_setup, enemy_state};
use gizmo::gizmo;
use line_of_sight::{
  line_of_sight_draw, line_of_sight_looking_at, line_of_sight_looking_at_draw, line_of_sight_shift,
  line_of_sight_update,
};
use movable::{path_direction, path_draw, path_follow, path_reset};
use navmesh::{navmesh_draw, navmesh_obstacle_draw, navmesh_setup};
use player::{player_animation, player_atlas_layout, player_path, player_setup, player_state};
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
      (
        camera_setup,
        player_setup,
        enemy_setup,
        tree_setup,
        navmesh_setup,
      ),
    )
    .add_systems(
      Update,
      (
        camera_pan,
        //
        player_animation,
        player_path,
        player_state,
        player_atlas_layout,
        //
        enemy_atlas_layout,
        enemy_state,
        //
        line_of_sight_update,
        line_of_sight_shift,
        line_of_sight_looking_at,
        line_of_sight_draw,
        //
        bounding_box_translation,
        //
        path_reset,
        path_follow,
        path_direction,
      ),
    )
    .add_systems(
      Update,
      (
        gizmo.run_if(debug),
        line_of_sight_looking_at_draw.run_if(debug),
        bounding_box_draw.run_if(debug),
        path_draw.run_if(debug),
        navmesh_draw.run_if(debug),
        navmesh_obstacle_draw.run_if(debug),
      ),
    )
    .add_systems(PostUpdate, y_sort)
    .run()
}

fn debug() -> bool {
  true
}
