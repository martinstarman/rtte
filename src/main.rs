pub mod component;
pub mod constants;
pub mod entity;
pub mod event;
pub mod mission;
pub mod resource;
pub mod system;

use bevy_ecs::{event::Events, query::With, schedule::Schedule, world::World};
use component::{
  enemy::EnemyComponent, polygon::PolygonComponent, position::PositionComponent,
  selection::SelectionComponent, size::SizeComponent, sprite::SpriteComponent, view::ViewComponent,
};
use event::{
  select_enemy_or_place_mark::SelectEnemyOrPlaceMark, select_or_move_player::SelectOrMovePlayer,
  select_or_stop_player::SelectOrStopPlayer,
};
use ggez::{
  event::{EventHandler, MouseButton},
  graphics::{Canvas, Color, DrawMode, DrawParam, Rect},
  input::keyboard::{KeyCode, KeyMods},
  mint::Point2,
  Context, ContextBuilder, GameError, GameResult,
};
use resource::{mark::Mark, target_area::TargetArea};
use std::path;
use system::{
  mark_in_view, movement, player_in_enemy_view, reach_target_area, select_enemy_or_place_mark,
  select_or_move_player, select_or_stop_player, view, view_current_direction,
  view_default_direction, view_shift,
};

const DEBUG: bool = true;
const WINDOW_WIDTH: f32 = 800.;
const WINDOW_HEIGHT: f32 = 600.;
const PAN_SPEED: f32 = 5.;

pub struct Game {
  world: World,
  schedule: Schedule,
  camera: Point2<f32>,
}

impl Game {
  pub fn new(ctx: &mut Context) -> GameResult<Game> {
    let mut world = World::default();

    let mission = mission::load("resources/demo.toml");

    for (i, player) in mission.player.iter().enumerate() {
      world.spawn(player.into(i, ctx));
    }

    for (i, enemy) in mission.enemy.iter().enumerate() {
      world.spawn(enemy.into(i, ctx));
    }

    for (i, polygon) in mission.polygon.iter().enumerate() {
      world.spawn(polygon.into(i, ctx));
    }

    for (i, image) in mission.image.iter().enumerate() {
      world.spawn(image.into(i, ctx));
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

    let game = Game {
      world,
      schedule,
      camera: Point2 { x: 0., y: 0. },
    };

    Ok(game)
  }
}

impl EventHandler<GameError> for Game {
  fn update(&mut self, ctx: &mut Context) -> GameResult {
    self.schedule.run(&mut self.world);

    for key in ctx.keyboard.pressed_keys() {
      match key {
        KeyCode::Left => self.camera.x -= PAN_SPEED,
        KeyCode::Right => self.camera.x += PAN_SPEED,
        KeyCode::Up => self.camera.y -= PAN_SPEED,
        KeyCode::Down => self.camera.y += PAN_SPEED,
        _ => (),
      }
    }

    let pos = ctx.mouse.position();

    if pos.x == 0. {
      self.camera.x -= PAN_SPEED;
    }

    if pos.x == WINDOW_WIDTH - 1. {
      self.camera.x += PAN_SPEED;
    }

    if pos.y == 0. {
      self.camera.y -= PAN_SPEED;
    }

    if pos.y == WINDOW_HEIGHT - 1. {
      self.camera.y += PAN_SPEED;
    }

    Ok(())
  }

  fn draw(&mut self, ctx: &mut Context) -> GameResult {
    let mut canvas = Canvas::from_frame(ctx, Color::from_rgb(255, 0, 255));

    draw_entity(self, ctx, &mut canvas, false); // Draw non Y indexed entities.
    draw_view(self, ctx, &mut canvas);
    draw_entity(self, ctx, &mut canvas, true); // Draw Y indexed entities.
    draw_mark(self, ctx, &mut canvas);

    if DEBUG {
      draw_entity_debug(self, ctx, &mut canvas);
      draw_target_area(self, ctx, &mut canvas);
    }

    canvas.finish(ctx)?;

    Ok(())
  }

  fn mouse_button_up_event(
    &mut self,
    ctx: &mut Context,
    btn: MouseButton,
    x: f32,
    y: f32,
  ) -> Result<(), GameError> {
    let x = x + self.camera.x;
    let y = y + self.camera.y;

    match btn {
      MouseButton::Left => {
        if ctx.keyboard.is_mod_active(KeyMods::SHIFT) {
          self.world.send_event(SelectEnemyOrPlaceMark { x, y });
        } else {
          self.world.send_event(SelectOrMovePlayer { x, y });
        }
      }
      MouseButton::Right => self.world.send_event(SelectOrStopPlayer::default()),
      _ => {}
    }

    Ok(())
  }
}

fn draw_entity(game: &mut Game, _ctx: &mut Context, canvas: &mut Canvas, ysorted: bool) {
  let mut query = game.world.query::<(&PositionComponent, &SizeComponent, &SpriteComponent)>();
  let mut entities: Vec<_> =
    query.iter_mut(&mut game.world).filter(|(_, _, sprite)| sprite.ysorted == ysorted).collect();

  if ysorted {
    entities.sort_by(|(a_position, a_size, _), (b_position, b_size, _)| {
      (a_position.y + a_size.height).partial_cmp(&(b_position.y + b_size.height)).unwrap()
    });
  }

  for (position, _, sprite) in entities {
    let dest = Point2 {
      x: position.x - game.camera.x,
      y: position.y - game.camera.y,
    };

    canvas.draw(&sprite.image, DrawParam::new().dest(dest));
  }
}

fn draw_mark(game: &mut Game, ctx: &mut Context, canvas: &mut Canvas) {
  if let Some(mark) = game.world.get_resource::<Mark>() {
    if let Some(position) = mark.position {
      let rect = Rect::new(position.x - 10., position.y - 10., 20., 20.);
      let mesh =
        ggez::graphics::Mesh::new_rectangle(ctx, DrawMode::stroke(1.), rect, Color::WHITE).unwrap();
      canvas.draw(&mesh, DrawParam::new().offset(game.camera));
    }
  }
}

fn draw_target_area(game: &mut Game, ctx: &mut Context, canvas: &mut Canvas) {
  if let Some(target_area) = game.world.get_resource::<TargetArea>() {
    let mesh = ggez::graphics::Mesh::new_rectangle(
      ctx,
      DrawMode::stroke(1.),
      target_area.rect,
      Color::GREEN,
    )
    .unwrap();
    canvas.draw(&mesh, DrawParam::new().offset(game.camera));
  }
}

fn draw_view(game: &mut Game, ctx: &mut Context, canvas: &mut Canvas) {
  let mut query =
    game.world.query_filtered::<(&SelectionComponent, &ViewComponent), With<EnemyComponent>>();

  for (selection, view) in query.iter_mut(&mut game.world) {
    if selection.active && view.polygon.len() >= 3 {
      let mesh = ggez::graphics::Mesh::new_polygon(
        ctx,
        DrawMode::fill(),
        &view.polygon[..],
        Color::from_rgba(255, 0, 0, 127),
      )
      .unwrap();
      canvas.draw(&mesh, DrawParam::new().offset(game.camera));
    }
  }
}

fn draw_entity_debug(game: &mut Game, ctx: &mut Context, canvas: &mut Canvas) {
  // rect
  let mut query = game.world.query::<(&PositionComponent, &SizeComponent)>();

  for (position, size) in query.iter_mut(&mut game.world) {
    let rect = Rect::new(position.x, position.y, size.width, size.height);
    let mesh =
      ggez::graphics::Mesh::new_rectangle(ctx, DrawMode::stroke(1.), rect, Color::WHITE).unwrap();
    canvas.draw(&mesh, DrawParam::new().offset(game.camera));
  }

  // polygon
  let mut query = game.world.query::<&PolygonComponent>();

  for object in query.iter_mut(&mut game.world) {
    if object.polygon.len() >= 3 {
      let points: Vec<Point2<f32>> =
        object.polygon.iter().map(|(p, _)| Point2 { x: p.x, y: p.y }).collect();

      let mesh =
        ggez::graphics::Mesh::new_polygon(ctx, DrawMode::stroke(1.), &points[..], Color::WHITE)
          .unwrap();
      canvas.draw(&mesh, DrawParam::new().offset(game.camera));
    }
  }
}

fn main() -> GameResult {
  let resource_dir = path::PathBuf::from("./resources");

  let context_builder = ContextBuilder::new("rtte", "rtte")
    .window_setup(ggez::conf::WindowSetup::default().title("rtte"))
    .window_mode(ggez::conf::WindowMode::default().dimensions(WINDOW_WIDTH, WINDOW_HEIGHT))
    .add_resource_path(resource_dir);

  let (mut ctx, event_loop) = context_builder.build()?;
  let state = Game::new(&mut ctx)?;

  ggez::input::mouse::set_cursor_grabbed(&mut ctx, true)?; // lock mouse to window

  ggez::event::run(ctx, event_loop, state)
}
