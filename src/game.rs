use crate::constants::PAN_SPEED;
use crate::event::{
  select_enemy_or_place_mark::SelectEnemyOrPlaceMark, select_or_move_player::SelectOrMovePlayer,
  select_or_stop_player::SelectOrStopPlayer, set_cursor::SetCursor,
};
use crate::mission;
use crate::resource::alarm::Alarm;
use crate::resource::cursor::{Cursor, CursorType};
use crate::resource::offset::Offset;
use crate::resource::physics::Physics;
use crate::resource::{mark::Mark, target_area::TargetArea};
use crate::system::animation::animation;
use crate::system::direction::direction;
use crate::system::draw_cursor::draw_cursor;
use crate::system::draw_entity::draw_entity;
use crate::system::draw_entity_debug::draw_entity_debug;
use crate::system::draw_entity_ysorted::draw_entity_ysorted;
use crate::system::draw_field_of_view::draw_field_of_view;
use crate::system::draw_fps::draw_fps;
use crate::system::draw_mark::draw_mark;
use crate::system::draw_navmesh::draw_navmesh;
use crate::system::draw_path::draw_path;
use crate::system::draw_target_area::draw_target_area;
use crate::system::field_of_view::field_of_view;
use crate::system::field_of_view_direction::field_of_view_direction;
use crate::system::field_of_view_movement_direction::field_of_view_movement_direction;
use crate::system::field_of_view_shift::field_of_view_shift;
use crate::system::mark_in_field_of_view::mark_in_field_of_view;
use crate::system::movement::movement;
use crate::system::physics::physics;
use crate::system::players_reach_target_area::players_reach_target_area;
use crate::system::reset_path::reset_path;
use crate::system::select_enemy_or_place_mark::select_enemy_or_place_mark;
use crate::system::select_or_move_players::select_or_move_players;
use crate::system::select_or_stop_players::select_or_stop_players;
use crate::system::set_cursor::set_cursor;
use crate::system::some_player_in_enemy_field_of_view::some_player_in_enemy_field_of_view;
use bevy_ecs::schedule::IntoSystemConfigs;
use bevy_ecs::{event::Events, schedule::Schedule, world::World};
use macroquad::input::{is_key_down, is_key_pressed, is_mouse_button_pressed, mouse_position};
use macroquad::math::Rect;
use macroquad::miniquad::window::request_quit;
use macroquad::miniquad::{KeyCode, MouseButton};
use macroquad::texture::load_texture;
use macroquad::window::{screen_height, screen_width};
use rapier2d::dynamics::RigidBodySet;
use rapier2d::geometry::ColliderSet;
use rapier2d::pipeline::QueryPipeline;

pub struct Game {
  world: World,
  schedule: Schedule,
}

impl Game {
  pub async fn new() -> Game {
    let mut world = World::default();

    let mut rigid_body_set = RigidBodySet::new();
    let mut collider_set = ColliderSet::new();
    let query_pipeline = QueryPipeline::new();

    let mission = mission::load("resources/demo.toml");

    for (i, player) in mission.player.iter().enumerate() {
      world.spawn(player.into(i, &mut rigid_body_set, &mut collider_set).await);
    }

    for (i, enemy) in mission.enemy.iter().enumerate() {
      world.spawn(enemy.into(i, &mut rigid_body_set, &mut collider_set).await);
    }

    for (i, object) in mission.object.iter().enumerate() {
      world.spawn(object.into(i).await);
    }

    world.insert_resource(Cursor::new().await);

    world.insert_resource(Mark {
      position: None,
      texture: load_texture("resources/mark.png").await.unwrap(),
    });
    world.insert_resource(Offset { x: 0., y: 0. });

    world.insert_resource(TargetArea {
      rect: Rect::new(500., 100., 100., 100.),
    });

    world.insert_resource(Physics {
      collider_set,
      query_pipeline,
      rigid_body_set,
    });

    world.insert_resource(Alarm::new().await);

    world.insert_resource(Events::<SelectEnemyOrPlaceMark>::default());
    world.insert_resource(Events::<SelectOrMovePlayer>::default());
    world.insert_resource(Events::<SelectOrStopPlayer>::default());
    world.insert_resource(Events::<SetCursor>::default());

    let mut schedule = Schedule::default();

    schedule.add_systems(movement.after(physics));
    schedule.add_systems(some_player_in_enemy_field_of_view);
    schedule.add_systems(players_reach_target_area);
    schedule.add_systems(field_of_view_direction);
    schedule.add_systems(field_of_view_movement_direction);
    schedule.add_systems(field_of_view_shift);
    schedule.add_systems(mark_in_field_of_view);
    schedule.add_systems(field_of_view);
    schedule.add_systems(select_enemy_or_place_mark);
    schedule.add_systems(select_or_move_players);
    schedule.add_systems(select_or_stop_players);
    schedule.add_systems(set_cursor);
    schedule.add_systems(
      draw_entity
        .before(draw_entity_ysorted)
        .before(draw_entity_debug)
        .before(draw_field_of_view)
        .before(draw_mark)
        .before(draw_target_area),
    );
    schedule.add_systems(draw_entity_ysorted.before(draw_entity_debug));
    schedule.add_systems(draw_entity_debug);
    schedule.add_systems(draw_field_of_view);
    schedule.add_systems(draw_mark);
    schedule.add_systems(draw_target_area);
    schedule.add_systems(
      draw_fps
        .after(draw_entity_ysorted)
        .after(draw_entity_debug)
        .after(draw_field_of_view)
        .after(draw_mark)
        .after(draw_target_area),
    );
    schedule.add_systems(
      draw_path
        .after(draw_entity_ysorted)
        .after(draw_entity_debug)
        .after(draw_field_of_view)
        .after(draw_mark)
        .after(draw_target_area),
    );
    schedule.add_systems(
      draw_navmesh
        .after(draw_entity_ysorted)
        .after(draw_entity_debug)
        .after(draw_field_of_view)
        .after(draw_mark)
        .after(draw_target_area),
    );
    schedule.add_systems(reset_path);
    schedule.add_systems(animation);
    schedule.add_systems(direction);

    schedule.add_systems(physics);
    schedule.add_systems(
      draw_cursor
        .after(draw_entity)
        .after(draw_entity_ysorted)
        .after(draw_entity_debug)
        .after(draw_field_of_view)
        .after(draw_fps)
        .after(draw_mark)
        .after(draw_navmesh)
        .after(draw_path)
        .after(draw_target_area)
        .after(draw_target_area),
    );

    Game { world, schedule }
  }

  pub fn update(&mut self) {
    self.schedule.run(&mut self.world);

    let mut offset = self.world.resource_mut::<Offset>();
    let (x, y) = mouse_position();

    if x == 0. || is_key_down(KeyCode::Left) {
      offset.x -= PAN_SPEED;
    }

    if x == screen_width() - 1. || is_key_down(KeyCode::Right) {
      offset.x += PAN_SPEED;
    }

    if y == 0. || is_key_down(KeyCode::Up) {
      offset.y -= PAN_SPEED;
    }

    if y == screen_height() - 1. || is_key_down(KeyCode::Down) {
      offset.y += PAN_SPEED;
    }

    if is_key_pressed(KeyCode::Escape) {
      request_quit()
    }

    let a = offset.x;
    let b = offset.y;

    if is_mouse_button_pressed(MouseButton::Left) {
      if is_key_down(KeyCode::LeftShift) {
        self.world.send_event(SelectEnemyOrPlaceMark { x: x + a, y: y + b });
      } else {
        self.world.send_event(SelectOrMovePlayer { x: x + a, y: y + b });
      }
    }

    if is_mouse_button_pressed(MouseButton::Right) {
      self.world.send_event(SelectOrStopPlayer {});
      self.world.send_event(SetCursor {
        cursor_type: CursorType::Default,
      });
    }

    if is_key_pressed(KeyCode::X) {
      self.world.send_event(SetCursor {
        cursor_type: CursorType::Knife,
      });
    }
  }
}
