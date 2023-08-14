/// Components.
pub mod components;

// ---------------------
pub mod entity;
pub mod geometry;

use crate::{
  entity::enemy::Enemy,
  geometry::{mesh::Mesh, vec2::Vec2},
};

use bevy_ecs::{schedule::Schedule, system::Query, world::World};
use components::{
  movable::Movable,
  player::{Player, PlayerBundle},
  position::Position,
  renderable::Renderable,
  selectable::Selectable,
  size::Size,
};
use ggez::{
  event::{self, EventHandler, MouseButton},
  graphics::{Canvas, Color, DrawMode, DrawParam, Image, Rect},
  input::keyboard::KeyCode,
  input::keyboard::KeyMods,
  mint::Point2,
  Context, ContextBuilder, GameError, GameResult,
};
use maths_rs::{distance, Vec2f};
use std::path;

pub struct Game {
  world: World,
  schedule: Schedule,
  offset: Vec2,
  scale: Vec2,
  mesh: Mesh,
  enemies: Vec<Enemy>,
}

const OFFSET_SPEED: f32 = 10.;

impl Game {
  pub fn new(_ctx: &mut Context) -> GameResult<Game> {
    let mut world = World::default();

    world.spawn(PlayerBundle {
      position: Position { x: 1., y: 1. },
      size: Size { w: 10., h: 23. },
      renderable: Renderable {
        sprite: String::from("/player.png"),
      },
      ..Default::default()
    });

    world.spawn(Position { x: 0., y: 20. });

    let mut schedule = Schedule::default();
    schedule.add_systems(movement);

    let game = Game {
      world,
      schedule,
      mesh: Mesh::new(800., 600.),
      offset: Vec2::default(),
      scale: Vec2::new(1., 1.),
      enemies: vec![],
    };

    Ok(game)
  }

  // handle LMB runtime mode click
  pub fn handle_runtime_mode_click(&mut self, ctx: &mut Context, _v: Vec2) {
    let mut query = self.world.query::<&mut Selectable>();

    // TODO
    for mut selectable in query.iter_mut(&mut self.world) {
      selectable.selected = !selectable.selected;
    }

    if ctx.keyboard.is_mod_active(KeyMods::SHIFT) {

      // // TODO: same code for every entity
      // for character in self.characters.iter_mut() {
      //   if character.get_rect().contains::<Point2<f32>>(v.into()) {
      //     character.is_selected = !character.is_selected
      //   }
      // }

      // for enemy in self.enemies.iter_mut() {
      //   if enemy.get_rect().contains::<Point2<f32>>(v.into()) {
      //     enemy.is_selected = !enemy.is_selected;
      //   }
      // }
    } else {
      // for character in self.characters.iter_mut() {
      //   if character.is_selected {
      //     character.path = self.mesh.find_path(character.pos, v);
      //   }
    }
  }
}

impl EventHandler<GameError> for Game {
  // main update fn
  fn update(&mut self, ctx: &mut Context) -> GameResult {
    self.schedule.run(&mut self.world);

    for key in ctx.keyboard.pressed_keys() {
      match key {
        KeyCode::Left => self.offset.x -= OFFSET_SPEED / self.scale.x,
        KeyCode::Right => self.offset.x += OFFSET_SPEED / self.scale.x,
        KeyCode::Up => self.offset.y -= OFFSET_SPEED / self.scale.x,
        KeyCode::Down => self.offset.y += OFFSET_SPEED / self.scale.x,
        _ => (),
      }
    }

    // // update characters
    // for character in self.characters.iter_mut() {
    //   character.update();
    // }

    // update enemies
    for enemy in self.enemies.iter_mut() {
      enemy.update();
      enemy.pov = self.mesh.get_pov(enemy.pos, enemy.pov_dest); // TODO: move to enemy.update()
    }

    Ok(())
  }

  // main draw fn
  fn draw(&mut self, ctx: &mut Context) -> GameResult {
    let mut canvas = Canvas::from_frame(ctx, Color::from_rgb(255, 0, 255));

    // draw mesh
    // TODO: draw background image/map
    self.mesh.draw(&mut canvas, ctx, &self);

    // draw player
    let mut query = self.world.query::<(&Player, &Position, &Size, &Renderable, &Selectable)>();
    for (_, position, size, renderable, selectable) in query.iter_mut(&mut self.world) {
      let image = Image::from_path(ctx, renderable.sprite.clone()).unwrap();
      let pos = Vec2::new(
        (position.x - self.offset.x) * self.scale.x,
        (position.y - self.offset.y) * self.scale.y,
      );
      canvas.draw(&image, DrawParam::new().dest(pos).scale(self.scale));

      // TODO: debug
      let rect = Rect::new(position.x, position.y, size.w, size.h);
      let color = if selectable.selected { Color::WHITE } else { Color::BLACK };
      let mesh =
        ggez::graphics::Mesh::new_rectangle(ctx, DrawMode::stroke(1.), rect, color).unwrap();
      canvas.draw(&mesh, DrawParam::new().offset(self.offset).scale(self.scale));
    }

    // draw characters
    // for character in &self.characters {
    //   character.draw(&mut canvas, ctx, &self);
    // }

    // draw enemies
    for enemy in &self.enemies {
      enemy.draw(&mut canvas, ctx, &self);
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
    let v = (Vec2::new(x, y) / self.scale.x) + self.offset;

    if self.mesh.rect.contains::<Point2<f32>>(v.into()) {
      match btn {
        MouseButton::Left => self.handle_runtime_mode_click(ctx, v),
        _ => {}
      }
    }

    Ok(())
  }
}

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
    }
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
