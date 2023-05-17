use crate::{geometry::vec2::Vec2, Mode, State};

use ggez::{
  graphics::{Canvas, Color, DrawMode, DrawParam, Mesh, Rect},
  Context,
};
use maths_rs::{distance, Vec2f};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Character {
  pub pos: Vec2,
  size: Vec2,
  #[serde(skip)]
  pub path: Vec<Vec2>,
  #[serde(skip)]
  pub is_selected: bool,
}

impl Default for Character {
  fn default() -> Self {
    Character::new(Vec2::new(200., 200.))
  }
}

impl Character {
  pub fn new(pos: Vec2) -> Self {
    Character {
      pos,
      size: Vec2::new(10., 10.),
      path: vec![],
      is_selected: false,
    }
  }

  pub fn update(&mut self) {
    self.walk();
  }

  pub fn draw(&self, canvas: &mut Canvas, ctx: &mut Context, state: &State) {
    // draw itself
    let color = if self.is_selected { Color::WHITE } else { Color::BLACK };

    let mesh =
      Mesh::new_circle(ctx, DrawMode::stroke(1.), self.pos, self.size.x / 2., 2., color).unwrap();
    canvas.draw(&mesh, DrawParam::new().offset(state.offset).scale(state.scale));

    if state.mode == Mode::Edit && self.path.len() >= 2 {
      let mesh =
        Mesh::new_polyline(ctx, DrawMode::stroke(1.), &self.path[..], Color::BLACK).unwrap();
      canvas.draw(&mesh, DrawParam::new().offset(state.offset).scale(state.scale));
    }
  }

  pub fn get_rect(&self) -> Rect {
    Rect::new(
      self.pos.x - (self.size.x / 2.),
      self.pos.y - (self.size.y / 2.),
      self.size.x,
      self.size.y,
    )
  }

  pub fn set_pos(&mut self, pos: Vec2) {
    self.pos = pos;
  }

  fn walk(&mut self) {
    if self.path.len() > 0 {
      let next = self.path[0];
      let dist = distance::<f32, Vec2f>(next.into(), self.pos.into());

      if dist < 1. {
        self.path.remove(0);
        self.pos = next;
      } else {
        let d = next - self.pos;
        self.pos = self.pos + (d / dist);
      }
    }
  }
}
