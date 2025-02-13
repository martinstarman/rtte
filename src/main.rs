mod animation;
mod camera;
mod console;
mod debug;
mod direction;
mod enemy;
mod line_of_sight;
mod movement;
mod navmesh;
mod object;
mod player;
mod selection;
mod ui;
mod utils;
mod ysort;

use bevy::{
  dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin},
  prelude::*,
  window::WindowResolution,
};
use bevy_minibuffer::prelude::*;
use camera::{camera_pan, camera_setup};
use console::console_setup;
use debug::{is_debug_enabled, toggle_debug, Debug};
use enemy::{enemy_animation, enemy_atlas_layout, enemy_setup, enemy_state};
use line_of_sight::{
  line_of_sight_draw, line_of_sight_looking_at, line_of_sight_looking_at_draw, line_of_sight_shift,
  line_of_sight_update,
};
use movement::{path_direction, path_draw, path_follow, path_reset};
use navmesh::navmesh_setup;
use object::object_setup;
use player::{player_animation, player_atlas_layout, player_path, player_setup, player_state};
use ui::ui_setup;
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
          text_config: TextFont {
            font_size: 20.,
            ..default()
          },
          enabled: false,
          ..default()
        },
      },
      VleueNavigatorPlugin,
      NavmeshUpdaterPlugin::<PrimitiveObstacle>::default(),
      MinibufferPlugins,
    ))
    .insert_resource(Debug::default())
    .add_systems(
      Startup,
      (
        camera_setup,
        player_setup,
        enemy_setup,
        navmesh_setup,
        console_setup,
        object_setup,
        ui_setup,
      ),
    )
    .add_systems(
      Update,
      (
        camera_pan,
        player_animation,
        player_path,
        player_state,
        player_atlas_layout,
        //
        enemy_animation,
        enemy_state,
        enemy_atlas_layout,
        //
        line_of_sight_update,
        line_of_sight_shift,
        line_of_sight_looking_at,
        line_of_sight_draw,
        //
        path_reset,
        path_follow,
        path_direction,
      ),
    )
    .add_systems(
      Update,
      (
        line_of_sight_looking_at_draw.run_if(is_debug_enabled),
        path_draw.run_if(is_debug_enabled),
      ),
    )
    .add_systems(PostUpdate, y_sort)
    .add_acts((player_setup, toggle_debug, BasicActs::default()))
    .run()
}
