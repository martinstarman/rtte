mod action;
mod animation;
mod camera;
mod console;
mod cursor;
mod debug;
mod direction;
mod enemy;
mod line_of_sight;
mod map;
mod movement;
mod navmesh;
mod object;
mod player;
mod selection;
mod ui;
mod utils;
mod ysort;

use crate::map::Map;
use bevy::{
  dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin},
  prelude::*,
  window::WindowResolution,
};
use bevy_minibuffer::prelude::*;
use camera::{camera_init, pan_camera};
use console::console_init;
use cursor::change_cursor_on_action_select;
use debug::{is_debug_enabled, toggle_debug, Debug};
use enemy::{
  enemy_animation_tick, enemy_init, enemy_reset_animation_on_state_change,
  enemy_update_atlas_layout_on_direction_or_state_change, enemy_update_state_on_movement_change,
};
use line_of_sight::{
  line_of_sight_draw_looking_at_position, line_of_sight_draw_polygon,
  line_of_sight_update_looking_at_position, line_of_sight_update_polygon_points,
  line_of_sight_update_shift,
};
use movement::{
  movement_draw_path, movement_entity_follow_path, movement_reset_path_on_empty,
  movement_update_entity_direction_on_change,
};
use navmesh::navmesh_init;
use object::object_init;
use player::{
  player_animation_tick, player_init, player_knife_melee_attack, player_set_or_reset_path_on_click,
  player_update_atlas_layout_on_direction_or_state_change, player_update_state_on_movement_change,
};
use ui::{
  actions::{ui_actions_init, ui_draw_actions, ui_toggle_actions_visibility},
  players::{ui_draw_players, ui_players_init, ui_update_players_on_player_added},
};
use vleue_navigator::{
  prelude::{NavmeshUpdaterPlugin, PrimitiveObstacle},
  VleueNavigatorPlugin,
};
use ysort::sort_by_y_index;

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
    .insert_resource(Map {
      width: 1200.0,
      height: 800.0,
    })
    .add_systems(
      Startup,
      (
        camera_init,
        player_init,
        enemy_init,
        navmesh_init,
        console_init,
        object_init,
        ui_players_init,
        ui_actions_init,
      ),
    )
    .add_systems(
      Update,
      (
        player_animation_tick,
        player_set_or_reset_path_on_click,
        player_update_state_on_movement_change,
        player_update_atlas_layout_on_direction_or_state_change,
        player_knife_melee_attack,
        //
        enemy_animation_tick,
        enemy_update_state_on_movement_change,
        enemy_update_atlas_layout_on_direction_or_state_change,
        enemy_reset_animation_on_state_change,
        //
        line_of_sight_update_polygon_points,
        line_of_sight_update_shift,
        line_of_sight_update_looking_at_position,
        line_of_sight_draw_polygon,
        line_of_sight_draw_looking_at_position.run_if(is_debug_enabled),
        //
        movement_reset_path_on_empty,
        movement_entity_follow_path,
        movement_update_entity_direction_on_change,
        movement_draw_path.run_if(is_debug_enabled),
      ),
    )
    .add_systems(
      Update,
      (
        ui_update_players_on_player_added,
        ui_draw_players,
        ui_toggle_actions_visibility,
        ui_draw_actions,
        //
        change_cursor_on_action_select,
        pan_camera,
      ),
    )
    .add_systems(PostUpdate, sort_by_y_index)
    .add_acts((toggle_debug, BasicActs::default()))
    .run()
}
