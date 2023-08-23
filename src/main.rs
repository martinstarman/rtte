/// Components.
pub mod components;
pub mod vec2;
use crate::vec2::Vec2;
use bevy_ecs::{component::ComponentId, schedule::Schedule, system::Query, world::World};
use components::{
  enemy::{Enemy, EnemyBundle},
  movable::Movable,
  object::{Object, ObjectBundle},
  player::{Player, PlayerBundle},
  position::Position,
  renderable::Renderable,
  selectable::Selectable,
  size::Size,
  view::View,
};
use ggez::{
  event::{self, EventHandler, MouseButton},
  graphics::{Canvas, Color, DrawMode, DrawParam, Image, Rect},
  input::keyboard::KeyCode,
  mint::Point2,
  Context, ContextBuilder, GameError, GameResult,
};
use maths_rs::{distance, Vec2f};
use std::{f32::consts::PI, path};

const WINDOW_WIDTH: f32 = 800.;
const WINDOW_HEIGHT: f32 = 600.;
const PAN_SPEED: f32 = 5.;
const ONE_DEGREE: f32 = PI / 180.;

/// Game data.
pub struct Game {
  /// Game world.
  world: World,

  /// Bevy ECS scheduler.
  schedule: Schedule,

  /// Camera position.
  camera: Vec2,
}

/// Game implementation.
impl Game {
  /// Create new game.
  pub fn new(ctx: &mut Context) -> GameResult<Game> {
    let mut world = World::default();

    world.spawn(ObjectBundle {
      position: Position { x: 0., y: 0. },
      size: Size { w: 1000., h: 800. },
      renderable: Renderable {
        sprite: Image::from_path(ctx, "/ground.png").unwrap(),
        y_indexed: false,
      },
      object: Object {
        poly: vec![],
        poly_type: components::object::PolyType::GROUND,
      },
    });

    world.spawn(ObjectBundle {
      position: Position { x: 250., y: 200. },
      size: Size { w: 160., h: 236. },
      renderable: Renderable {
        sprite: Image::from_path(ctx, "/block.png").unwrap(),
        y_indexed: true,
      },
      object: Object {
        poly: vec![
          Vec2::new(128., 236.),
          Vec2::new(160., 219.),
          Vec2::new(32., 154.),
          Vec2::new(0., 171.),
        ],
        poly_type: components::object::PolyType::GROUND,
      },
    });

    world.spawn(PlayerBundle {
      position: Position { x: 1., y: 1. },
      size: Size { w: 10., h: 23. },
      renderable: Renderable {
        sprite: Image::from_path(ctx, "/player.png").unwrap(),
        y_indexed: true,
      },
      movable: Movable {
        path: vec![],
        path_default: vec![],
      },
      player: Player {
        id: ComponentId::new(1),
      },
      selectable: Selectable { selected: false },
    });

    world.spawn(PlayerBundle {
      position: Position { x: 30., y: 1. },
      size: Size { w: 10., h: 23. },
      renderable: Renderable {
        sprite: Image::from_path(ctx, "/player.png").unwrap(),
        y_indexed: true,
      },
      movable: Movable {
        path: vec![],
        path_default: vec![],
      },
      player: Player {
        id: ComponentId::new(2),
      },
      selectable: Selectable { selected: false },
    });

    world.spawn(EnemyBundle {
      position: Position { x: 100., y: 100. },
      size: Size { w: 10., h: 23. },
      renderable: Renderable {
        sprite: Image::from_path(ctx, "/player.png").unwrap(),
        y_indexed: true,
      },
      movable: Movable {
        path: vec![],
        path_default: vec![Vec2::new(200., 200.), Vec2::new(100., 100.)],
      },
      view: View { x: 125., y: 100. },
      enemy: Enemy {},
      selectable: Selectable { selected: false },
    });

    let mut schedule = Schedule::default();

    schedule.add_systems(movement);
    schedule.add_systems(view);

    let game = Game {
      world,
      schedule,
      camera: Vec2::default(),
    };

    Ok(game)
  }
}

/// ggez event handler for Game.
impl EventHandler<GameError> for Game {
  /// Every tick update.
  fn update(&mut self, ctx: &mut Context) -> GameResult {
    // Run bevy_ecs systems.
    self.schedule.run(&mut self.world);

    // Change camera position by arrow keys.
    for key in ctx.keyboard.pressed_keys() {
      match key {
        KeyCode::Left => self.camera.x -= PAN_SPEED,
        KeyCode::Right => self.camera.x += PAN_SPEED,
        KeyCode::Up => self.camera.y -= PAN_SPEED,
        KeyCode::Down => self.camera.y += PAN_SPEED,
        _ => (),
      }
    }

    // Change camera position by mouse.
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

  /// Every tick draw.
  fn draw(&mut self, ctx: &mut Context) -> GameResult {
    // Reset canvas.
    let mut canvas = Canvas::from_frame(ctx, Color::from_rgb(255, 0, 255));

    draw_entity(self, ctx, &mut canvas);
    draw_object_poly(self, ctx, &mut canvas);

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
    _ctx: &mut Context,
    btn: MouseButton,
    x: f32,
    y: f32,
  ) -> Result<(), GameError> {
    let x = x + self.camera.x;
    let y = y + self.camera.y;

    match btn {
      MouseButton::Left => select_or_move_player(self, x, y),
      MouseButton::Right => select_or_stop_player(self, x, y),
      _ => {}
    }

    Ok(())
  }
}

/// Draw entity.
fn draw_entity(game: &mut Game, ctx: &mut Context, canvas: &mut Canvas) {
  let mut query = game.world.query::<(&Position, &Size, &Renderable)>();
  let mut entities: Vec<_> = query.iter_mut(&mut game.world).collect();

  // Sort by Y index.
  entities.sort_by(|(a_position, a_size, a_renderable), (b_position, b_size, b_renderable)| {
    let a_y_index = if a_renderable.y_indexed { a_position.y + a_size.h } else { 0. };
    let b_y_index = if b_renderable.y_indexed { b_position.y + b_size.h } else { 0. };

    (a_y_index).partial_cmp(&(&b_y_index)).unwrap()
  });

  for (position, size, renderable) in entities {
    let dest = Vec2::new(position.x - game.camera.x, position.y - game.camera.y);
    let rect = Rect::new(position.x, position.y, size.w, size.h);
    let mesh =
      ggez::graphics::Mesh::new_rectangle(ctx, DrawMode::stroke(1.), rect, Color::BLACK).unwrap();

    canvas.draw(&renderable.sprite, DrawParam::new().dest(dest));
    canvas.draw(&mesh, DrawParam::new().offset(game.camera));
  }
}

/// Draw objects polygon.
fn draw_object_poly(game: &mut Game, ctx: &mut Context, canvas: &mut Canvas) {
  let mut query = game.world.query::<(&Object, &Position)>();
  let mut points: Vec<Vec2> = vec![];

  for (object, position) in query.iter_mut(&mut game.world) {
    if object.poly.len() >= 3 {
      for v in &object.poly {
        points.push(Vec2::new(position.x + v.x, position.y + v.y));
      }

      let mesh =
        ggez::graphics::Mesh::new_polygon(ctx, DrawMode::stroke(1.), &points[..], Color::WHITE)
          .unwrap();
      canvas.draw(&mesh, DrawParam::new().offset(game.camera));
    }
  }
}

/// Left mouse button click handler.
fn select_or_move_player(game: &mut Game, x: f32, y: f32) {
  let mut selected_player_id: Option<ComponentId> = None;

  // Try to select player.
  let mut query = game.world.query::<(&Player, &mut Selectable, &Position, &Size)>();

  for (player, mut selectable, position, size) in query.iter_mut(&mut game.world) {
    let rect = Rect::new(position.x, position.y, size.w, size.h);

    if rect.contains(Point2 { x, y }) {
      selectable.selected = true;
      selected_player_id = Some(player.id);
    }
  }

  // Deselect all players if some was selected.
  if let Some(id) = selected_player_id {
    let mut query = game.world.query::<(&Player, &mut Selectable)>();

    for (player, mut selectable) in query.iter_mut(&mut game.world) {
      if player.id != id {
        selectable.selected = false;
      }
    }
  }

  // Set path to selected player when no player was selected.
  if selected_player_id.is_none() {
    let mut query = game.world.query::<(&Player, &Selectable, &mut Movable)>();

    for (_, selectable, mut movable) in query.iter_mut(&mut game.world) {
      if selectable.selected {
        movable.path = vec![Vec2::new(x, y)]; // TODO: check where is clicked
      }
    }
  }
}

/// Right mouse button click handler.
fn select_or_stop_player(game: &mut Game, _x: f32, _y: f32) {
  // TODO: multiple player selection

  // Stop player movement.
  let mut query = game.world.query::<(&Player, &mut Movable)>();

  for (_, mut movable) in query.iter_mut(&mut game.world) {
    movable.path = vec![];
  }
}

/// Entity movement.
fn movement(mut query: Query<(&mut Movable, &mut Position)>) {
  for (mut movable, mut position) in &mut query {
    if movable.path.len() > 0 {
      let next = movable.path[0];
      let dist = distance::<f32, Vec2f>(next.into(), Vec2f::new(position.x, position.y));

      if dist < 1. {
        movable.path.remove(0);

        position.x = next.x;
        position.y = next.y;
      } else {
        let dx = next.x - position.x;
        let dy = next.y - position.y;

        position.x = position.x + (dx / dist);
        position.y = position.y + (dy / dist);
      }
    } else {
      if movable.path_default.len() > 0 {
        movable.path = movable.path_default.clone();
      }
    }
  }
}

/// Entity view.
fn view(mut query: Query<(&mut View, &Position)>) {
  for (mut view, position) in &mut query {
    let dx = view.x - position.x;
    let dy = view.y - position.y;

    // TODO: limits
    view.x = f32::cos(ONE_DEGREE) * dx - f32::sin(ONE_DEGREE) * dy + position.x;
    view.y = f32::sin(ONE_DEGREE) * dx + f32::cos(ONE_DEGREE) * dy + position.y;
  }
}

/// Main function.
fn main() -> GameResult {
  let resource_dir = path::PathBuf::from("./resources");

  let context_builder = ContextBuilder::new("rtte", "rtte")
    .window_setup(ggez::conf::WindowSetup::default().title("rtte"))
    .window_mode(ggez::conf::WindowMode::default().dimensions(WINDOW_WIDTH, WINDOW_HEIGHT))
    .add_resource_path(resource_dir);

  let (mut ctx, event_loop) = context_builder.build()?;
  let state = Game::new(&mut ctx)?;

  // Lock mouse to window.
  ggez::input::mouse::set_cursor_grabbed(&mut ctx, true)?;

  event::run(ctx, event_loop, state)
}
