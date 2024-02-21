use crate::constants::PAN_SPEED;
use crate::event::{
  select_enemy_or_place_mark::SelectEnemyOrPlaceMark, select_or_move_player::SelectOrMovePlayer,
  select_or_stop_player::SelectOrStopPlayer,
};
use crate::mission;
use crate::resource::offset::Offset;
use crate::resource::{mark::Mark, target_area::TargetArea};
use crate::system::{
  draw_entity, draw_entity_debug, draw_entity_ysorted, draw_mark, draw_target, draw_view,
  mark_in_view, movement, player_in_enemy_view, reach_target_area, select_enemy_or_place_mark,
  select_or_move_player, select_or_stop_player, view, view_current_direction,
  view_default_direction, view_shift,
};
use bevy_ecs::schedule::IntoSystemConfigs;
use bevy_ecs::{event::Events, schedule::Schedule, world::World};
use macroquad::input::{is_key_down, is_key_pressed, is_mouse_button_pressed, mouse_position};
use macroquad::math::Rect;
use macroquad::miniquad::window::request_quit;
use macroquad::miniquad::{KeyCode, MouseButton};
use macroquad::window::{screen_height, screen_width};

pub struct Game {
  world: World,
  schedule: Schedule,
}

impl Game {
  pub async fn new() -> Game {
    let mut world = World::default();

    let mission = mission::load("resources/demo.toml");

    for (i, player) in mission.player.iter().enumerate() {
      world.spawn(player.into(i).await);
    }

    for (i, enemy) in mission.enemy.iter().enumerate() {
      world.spawn(enemy.into(i).await);
    }

    for (i, polygon) in mission.polygon.iter().enumerate() {
      world.spawn(polygon.into(i));
    }

    for (i, image) in mission.image.iter().enumerate() {
      world.spawn(image.into(i).await);
    }

    world.insert_resource(Mark { position: None });
    world.insert_resource(Offset { x: 0., y: 0. }); // TODO: toml

    world.insert_resource(TargetArea {
      rect: Rect::new(500., 100., 100., 100.), // TODO: toml
    });

    world.insert_resource(Events::<SelectEnemyOrPlaceMark>::default());
    world.insert_resource(Events::<SelectOrMovePlayer>::default());
    world.insert_resource(Events::<SelectOrStopPlayer>::default());

    let mut schedule = Schedule::default();

    schedule.add_systems(movement::run);
    schedule.add_systems(player_in_enemy_view::run);
    schedule.add_systems(reach_target_area::run);
    schedule.add_systems(view_current_direction::run);
    schedule.add_systems(view_default_direction::run);
    schedule.add_systems(view_shift::run);
    schedule.add_systems(mark_in_view::run);
    schedule.add_systems(view::run);
    schedule.add_systems(select_enemy_or_place_mark::run);
    schedule.add_systems(select_or_move_player::run);
    schedule.add_systems(select_or_stop_player::run);
    schedule.add_systems(
      draw_entity::run
        .before(draw_entity_ysorted::run)
        .before(draw_entity_debug::run)
        .before(draw_view::run)
        .before(draw_mark::run)
        .before(draw_target::run),
    );
    schedule.add_systems(draw_entity_ysorted::run.before(draw_entity_debug::run));
    schedule.add_systems(draw_entity_debug::run);
    schedule.add_systems(draw_view::run);
    schedule.add_systems(draw_mark::run);
    schedule.add_systems(draw_target::run);

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
      self.world.send_event(SelectOrStopPlayer::default());
    }
  }
}
