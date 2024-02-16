use crate::component::{
  enemy::EnemyComponent, polygon::PolygonComponent, position::PositionComponent,
  selection::SelectionComponent, size::SizeComponent, sprite::SpriteComponent, view::ViewComponent,
};
use crate::constants::{DEBUG, PAN_SPEED, WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::event::{
  select_enemy_or_place_mark::SelectEnemyOrPlaceMark, select_or_move_player::SelectOrMovePlayer,
  select_or_stop_player::SelectOrStopPlayer,
};
use crate::mission;
use crate::resource::{mark::Mark, target_area::TargetArea};
use crate::system::{
  mark_in_view, movement, player_in_enemy_view, reach_target_area, select_enemy_or_place_mark,
  select_or_move_player, select_or_stop_player, view, view_current_direction,
  view_default_direction, view_shift,
};
use bevy_ecs::{event::Events, query::With, schedule::Schedule, world::World};
use macroquad::prelude::*;

pub struct Game {
  world: World,
  schedule: Schedule,
  camera: Vec2,
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

    world.insert_resource(TargetArea {
      rect: Rect::new(500., 100., 100., 100.),
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

    Game {
      world,
      schedule,
      camera: Vec2 { x: 0., y: 0. },
    }
  }

  pub fn update(&mut self) {
    self.schedule.run(&mut self.world);

    let (x, y) = mouse_position();

    if x == 0. {
      self.camera.x -= PAN_SPEED;
    }

    if x == (WINDOW_WIDTH - 1) as f32 {
      self.camera.x += PAN_SPEED;
    }

    if y == 0. {
      self.camera.y -= PAN_SPEED;
    }

    if y == (WINDOW_HEIGHT - 1) as f32 {
      self.camera.y += PAN_SPEED;
    }

    if is_key_down(KeyCode::Left) {
      self.camera.x -= PAN_SPEED;
    }

    if is_key_down(KeyCode::Right) {
      self.camera.x += PAN_SPEED;
    }

    if is_key_down(KeyCode::Up) {
      self.camera.y -= PAN_SPEED;
    }

    if is_key_down(KeyCode::Down) {
      self.camera.y += PAN_SPEED;
    }

    if is_key_pressed(KeyCode::Escape) {
      // TODO
    }

    if is_mouse_button_pressed(MouseButton::Left) {
      if is_key_down(KeyCode::LeftShift) {
        self.world.send_event(SelectEnemyOrPlaceMark { x, y });
      } else {
        self.world.send_event(SelectOrMovePlayer { x, y });
      }
    }

    if is_mouse_button_pressed(MouseButton::Right) {
      self.world.send_event(SelectOrStopPlayer::default());
    }
  }

  pub fn draw(&mut self) {
    self.draw_entity(false); // Draw non Y indexed entities.
    self.draw_view();
    self.draw_entity(true); // Draw Y indexed entities.
    self.draw_mark();

    if DEBUG {
      self.draw_entity_debug();
      self.draw_target_area();
    }
  }

  fn draw_entity(&mut self, ysorted: bool) {
    let mut query = self.world.query::<(&PositionComponent, &SizeComponent, &SpriteComponent)>();
    let mut entities: Vec<_> =
      query.iter_mut(&mut self.world).filter(|(_, _, sprite)| sprite.ysorted == ysorted).collect();

    if ysorted {
      entities.sort_by(|(a_position, a_size, _), (b_position, b_size, _)| {
        (a_position.y + a_size.height).partial_cmp(&(b_position.y + b_size.height)).unwrap()
      });
    }

    for (position, _, sprite) in entities {
      let dest = Vec2 {
        x: position.x - self.camera.x,
        y: position.y - self.camera.y,
      };

      draw_texture(&sprite.image, dest.x, dest.y, WHITE);
    }
  }

  fn draw_mark(&mut self) {
    if let Some(mark) = self.world.get_resource::<Mark>() {
      if let Some(position) = mark.position {
        draw_rectangle_lines(position.x - 10., position.y - 10., 20., 20., 1., WHITE);
      }
    }
  }

  fn draw_target_area(&mut self) {
    if let Some(target_area) = self.world.get_resource::<TargetArea>() {
      draw_rectangle_lines(
        target_area.rect.x,
        target_area.rect.y,
        target_area.rect.w,
        target_area.rect.h,
        1.,
        WHITE,
      );
    }
  }

  fn draw_view(&mut self) {
    let mut query =
      self.world.query_filtered::<(&SelectionComponent, &ViewComponent), With<EnemyComponent>>();

    for (selection, view) in query.iter_mut(&mut self.world) {
      if selection.active && view.polygon.len() >= 3 {
        // TODO
        // let mesh = ggez::graphics::Mesh::new_polygon(
        //   ctx,
        //   DrawMode::fill(),
        //   &view.polygon[..],
        //   Color::from_rgba(255, 0, 0, 127),
        // )
        // .unwrap();
        // canvas.draw(&mesh, DrawParam::new().offset(game.camera));
      }
    }
  }

  fn draw_entity_debug(&mut self) {
    // rect
    let mut query = self.world.query::<(&PositionComponent, &SizeComponent)>();

    for (position, size) in query.iter_mut(&mut self.world) {
      draw_rectangle_lines(position.x, position.y, size.width, size.height, 1., WHITE);
    }

    // polygon
    let mut query = self.world.query::<&PolygonComponent>();

    for object in query.iter_mut(&mut self.world) {
      if object.polygon.len() >= 3 {
        // TODO
        // let points: Vec<Vec2> = object.polygon.iter().map(|(p, _)| Vec2::new(p.x, p.y)).collect();

        // let mesh =
        //   ggez::graphics::Mesh::new_polygon(ctx, DrawMode::stroke(1.), &points[..], Color::WHITE)
        //     .unwrap();
        // canvas.draw(&mesh, DrawParam::new().offset(game.camera));
      }
    }
  }
}
