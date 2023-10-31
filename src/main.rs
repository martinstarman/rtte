pub mod component;
pub mod entity;
pub mod mission;
pub mod resource;
pub mod system;

use bevy_ecs::{component::ComponentId, query::With, schedule::Schedule, world::World};
use component::{
  enemy::Enemy,
  movement::Movement,
  object::{Object, PolygonType},
  player::Player,
  position::Position,
  selection::Selection,
  size::Size,
  sprite::Sprite,
  view::View,
};
use ggez::{
  event::{self, EventHandler, MouseButton},
  graphics::{Canvas, Color, DrawMode, DrawParam, Image, Rect},
  input::keyboard::{KeyCode, KeyMods},
  mint::Point2,
  Context, ContextBuilder, GameError, GameResult,
};
use resource::mark::Mark;
use std::path;

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
      world.spawn(player.to_component(i, ctx));
    }

    for (i, enemy) in mission.enemy.iter().enumerate() {
      world.spawn(enemy.to_component(i, ctx));
    }

    world.spawn(entity::object::new(
      Position { x: 0., y: 0. },
      Image::from_path(ctx, "/ground.png").unwrap(),
      vec![
        Point2 { x: 0., y: 0. },
        Point2 { x: 1000., y: 0. },
        Point2 { x: 1000., y: 800. },
        Point2 { x: 0., y: 800. },
      ],
      PolygonType::GROUND,
    ));

    world.spawn(entity::object::new(
      Position { x: 250., y: 200. },
      Image::from_path(ctx, "/block.png").unwrap(),
      vec![
        Point2 { x: 128., y: 236. },
        Point2 { x: 160., y: 219. },
        Point2 { x: 32., y: 154. },
        Point2 { x: 0., y: 171. },
      ],
      PolygonType::BLOCK,
    ));

    world.insert_resource(Mark {
      active: false,
      x: 0.,
      y: 0.,
    });

    let mut schedule = Schedule::default();

    schedule.add_systems(system::movement::update);
    schedule.add_systems(system::view::update_shift);
    schedule.add_systems(system::view::update_current_direction);
    schedule.add_systems(system::view::update_default_direction);
    schedule.add_systems(system::view::mark_in_view);
    schedule.add_systems(system::view::update);

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
    }

    canvas.finish(ctx)?;

    Ok(())
  }

  fn mouse_button_down_event(
    &mut self,
    _ctx: &mut Context,
    _button: MouseButton,
    _x: f32,
    _y: f32,
  ) -> Result<(), GameError> {
    // TODO: multiple player selection

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
          select_enemy_or_place_mark(self, x, y);
        } else {
          select_or_move_player(self, x, y);
        }
      }
      MouseButton::Right => select_or_stop_player(self, x, y),
      _ => {}
    }

    Ok(())
  }
}

fn draw_entity(game: &mut Game, _ctx: &mut Context, canvas: &mut Canvas, ysorted: bool) {
  let mut query = game.world.query::<(&Position, &Size, &Sprite)>();
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
    if mark.active {
      let rect = Rect::new(mark.x - 10., mark.y - 10., 20., 20.);
      let mesh =
        ggez::graphics::Mesh::new_rectangle(ctx, DrawMode::stroke(1.), rect, Color::WHITE).unwrap();
      canvas.draw(&mesh, DrawParam::new().offset(game.camera));
    }
  }
}

fn draw_view(game: &mut Game, ctx: &mut Context, canvas: &mut Canvas) {
  let mut query = game.world.query_filtered::<(&Selection, &View), With<Enemy>>();

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
  let mut query = game.world.query::<(&Position, &Size)>();

  for (position, size) in query.iter_mut(&mut game.world) {
    let rect = Rect::new(position.x, position.y, size.width, size.height);
    let mesh =
      ggez::graphics::Mesh::new_rectangle(ctx, DrawMode::stroke(1.), rect, Color::WHITE).unwrap();
    canvas.draw(&mesh, DrawParam::new().offset(game.camera));
  }

  // polygon
  let mut query = game.world.query::<(&Object, &Position)>();

  for (object, position) in query.iter_mut(&mut game.world) {
    let points: Vec<Point2<f32>> = object
      .polygon
      .iter()
      .map(|(p1, _)| Point2 {
        x: p1.x + position.x,
        y: p1.y + position.y,
      })
      .collect();

    let mesh =
      ggez::graphics::Mesh::new_polygon(ctx, DrawMode::stroke(1.), &points[..], Color::WHITE)
        .unwrap();
    canvas.draw(&mesh, DrawParam::new().offset(game.camera));
  }
}

fn select_enemy_or_place_mark(game: &mut Game, x: f32, y: f32) {
  let mut current_selected_enemy_id: Option<ComponentId> = None;
  let mut new_enemy_selected: bool = false;

  // try to select enemy
  let mut query = game.world.query::<(&Enemy, &mut Selection, &Position, &Size)>();

  for (enemy, mut selection, position, size) in query.iter_mut(&mut game.world) {
    let rect = Rect::new(position.x, position.y, size.width, size.height);

    if selection.active {
      current_selected_enemy_id = Some(enemy.id);
    }

    if rect.contains(Point2 { x, y }) && !selection.active {
      selection.active = true;
      new_enemy_selected = true;
    }
  }

  // deselect current selected enemy
  if new_enemy_selected {
    if let Some(id) = current_selected_enemy_id {
      let mut query = game.world.query::<(&Enemy, &mut Selection)>();

      for (enemy, mut selection) in query.iter_mut(&mut game.world) {
        if enemy.id == id {
          selection.active = false;
        }
      }
    }
  } else {
    if let Some(mut mark) = game.world.get_resource_mut::<Mark>() {
      mark.active = true;
      mark.x = x;
      mark.y = y;
    }
  }
}

fn select_or_move_player(game: &mut Game, x: f32, y: f32) {
  let mut selected_player_id: Option<ComponentId> = None;

  // try to select player
  let mut query = game.world.query::<(&Player, &mut Selection, &Position, &Size)>();

  for (player, mut selection, position, size) in query.iter_mut(&mut game.world) {
    let rect = Rect::new(position.x, position.y, size.width, size.height);

    if rect.contains(Point2 { x, y }) {
      selection.active = true;
      selected_player_id = Some(player.id);
    }
  }

  // deselect all players if some was selected
  if let Some(id) = selected_player_id {
    let mut query = game.world.query::<(&Player, &mut Selection)>();

    for (player, mut selection) in query.iter_mut(&mut game.world) {
      if player.id != id {
        selection.active = false;
      }
    }
  }

  // set path to selected player when no player was selected
  if selected_player_id.is_none() {
    let mut query = game.world.query::<(&Player, &Selection, &mut Movement)>();

    for (_, selection, mut movement) in query.iter_mut(&mut game.world) {
      if selection.active {
        movement.current_path = vec![Point2 { x, y }]; // TODO: is target walkable?
      }
    }
  }
}

fn select_or_stop_player(game: &mut Game, _x: f32, _y: f32) {
  // TODO: multiple player selection

  // stop player movement
  let mut query = game.world.query::<(&Player, &mut Movement)>();

  for (_, mut movement) in query.iter_mut(&mut game.world) {
    movement.current_path = vec![];
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

  event::run(ctx, event_loop, state)
}
