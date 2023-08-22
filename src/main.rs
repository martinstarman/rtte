/// Components.
pub mod components;

pub mod vec2;
use crate::vec2::Vec2;

use bevy_ecs::{schedule::Schedule, system::Query, world::World};
use components::{
  enemy::EnemyBundle,
  movable::Movable,
  object::{Object, ObjectBundle, PolyType},
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

pub struct Game {
  world: World,
  schedule: Schedule,
  offset: Vec2,
}

const ONE_DEGREE: f32 = PI / 180.;
const OFFSET_SPEED: f32 = 10.;

impl Game {
  pub fn new(ctx: &mut Context) -> GameResult<Game> {
    let mut world = World::default();

    world.spawn(ObjectBundle {
      position: Position { x: 0., y: 0. },
      size: Size { w: 1000., h: 800. },
      renderable: Renderable {
        sprite: Some(Image::from_path(ctx, "/ground.png").unwrap()),
      },
      ..Default::default()
    });

    world.spawn(ObjectBundle {
      position: Position { x: 250., y: 200. },
      size: Size { w: 160., h: 236. },
      renderable: Renderable {
        sprite: Some(Image::from_path(ctx, "/block.png").unwrap()),
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
        sprite: Some(Image::from_path(ctx, "/player.png").unwrap()),
      },
      ..Default::default()
    });

    world.spawn(EnemyBundle {
      position: Position { x: 100., y: 100. },
      size: Size { w: 10., h: 23. },
      renderable: Renderable {
        sprite: Some(Image::from_path(ctx, "/player.png").unwrap()),
      },
      movable: Movable {
        path: vec![],
        path_default: vec![Vec2::new(200., 200.), Vec2::new(100., 100.)],
      },
      view: View { x: 125., y: 100. },
      ..Default::default()
    });

    let mut schedule = Schedule::default();

    schedule.add_systems(movement);
    schedule.add_systems(view);

    let game = Game {
      world,
      schedule,
      offset: Vec2::default(),
    };

    Ok(game)
  }
}

impl EventHandler<GameError> for Game {
  // main update fn
  fn update(&mut self, ctx: &mut Context) -> GameResult {
    self.schedule.run(&mut self.world);

    for key in ctx.keyboard.pressed_keys() {
      match key {
        KeyCode::Left => self.offset.x -= OFFSET_SPEED,
        KeyCode::Right => self.offset.x += OFFSET_SPEED,
        KeyCode::Up => self.offset.y -= OFFSET_SPEED,
        KeyCode::Down => self.offset.y += OFFSET_SPEED,
        _ => (),
      }
    }

    Ok(())
  }

  fn draw(&mut self, ctx: &mut Context) -> GameResult {
    let mut canvas = Canvas::from_frame(ctx, Color::from_rgb(255, 0, 255));

    draw_entity(self, ctx, &mut canvas);
    draw_object_poly(self, ctx, &mut canvas);
    draw_enemy_view(self, ctx, &mut canvas);

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
    let v = Vec2::new(x + self.offset.x, y + self.offset.y);

    match btn {
      MouseButton::Left => on_left_mouse_button_click(self, ctx, v),
      MouseButton::Right => on_right_mouse_button_click(self, ctx, v),
      _ => {}
    }

    Ok(())
  }
}

/// Draw entity.
// TODO: sort by Y index
fn draw_entity(game: &mut Game, ctx: &mut Context, canvas: &mut Canvas) {
  let mut query = game.world.query::<(&Position, &Size, &Renderable)>();

  for (position, size, renderable) in query.iter_mut(&mut game.world) {
    let dest = Vec2::new(
      position.x - game.offset.x,
      position.y - game.offset.y,
    );
    let rect = Rect::new(position.x, position.y, size.w, size.h);
    let mesh =
      ggez::graphics::Mesh::new_rectangle(ctx, DrawMode::stroke(1.), rect, Color::BLACK).unwrap();

    canvas.draw(renderable.sprite.as_ref().unwrap(), DrawParam::new().dest(dest));
    canvas.draw(&mesh, DrawParam::new().offset(game.offset));
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
      canvas.draw(&mesh, DrawParam::new().offset(game.offset));
    }
  }
}

/// Draw enemy view.
fn draw_enemy_view(_game: &mut Game, _ctx: &mut Context, _canvas: &mut Canvas) {
  // TODO
}

/// Left mouse button click handler.
fn on_left_mouse_button_click(game: &mut Game, _ctx: &mut Context, v: Vec2) {
  let mut query = game.world.query::<&Object>();
  let mut objects: Vec<Object> = vec![];

  for object in query.iter_mut(&mut game.world) {
    if object.poly_type == PolyType::BLOCK || object.poly_type == PolyType::TRANSPARENT {
      objects.push(object.clone());
    }
  }

  let mut query = game.world.query::<(&Player, &Position, &Selectable, &mut Movable)>();

  for (_, _position, selectable, mut movable) in query.iter_mut(&mut game.world) {
    if selectable.selected {
      // TODO: check if intersects some block poly
      movable.path = vec![v];
    }
  }
}

/// Right mouse button click handler.
fn on_right_mouse_button_click(game: &mut Game, _ctx: &mut Context, v: Vec2) {
  let mut query = game.world.query::<(&mut Selectable, &Position, &Size)>();

  for (mut selectable, position, size) in query.iter_mut(&mut game.world) {
    let rect = Rect::new(position.x, position.y, size.w, size.h);

    if rect.contains::<Point2<f32>>(v.into()) {
      selectable.selected = !selectable.selected;
    }
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
        position.y = next.x;
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
    .window_mode(ggez::conf::WindowMode::default().dimensions(800., 600.))
    .add_resource_path(resource_dir);

  let (mut ctx, event_loop) = context_builder.build()?;
  let state = Game::new(&mut ctx)?;

  event::run(ctx, event_loop, state)
}
