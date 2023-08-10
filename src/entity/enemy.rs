use crate::{geometry::vec2::Vec2, resource::Resource, Mode, State};

use ggez::{
  graphics::{Canvas, Color, DrawMode, DrawParam, Mesh, Rect},
  Context,
};
use maths_rs::{distance, Vec2f};
use serde::{Deserialize, Serialize};
use std::f32::consts::PI;

const ONE_DEGREE: f32 = PI / 180.;
const VIEW_DISTANCE: f32 = 200.;

#[derive(Serialize, Deserialize)]
pub struct Enemy {
  pub pos: Vec2,
  size: Vec2,
  pub path_default: Vec<Vec2>,
  #[serde(skip)]
  pub pov: Vec<Vec2>,
  #[serde(skip)]
  pub pov_dest: Vec2,
  #[serde(skip)]
  pub is_selected: bool,
  #[serde(skip)]
  pub path: Vec<Vec2>,
  pub res_path: String,
}

impl Default for Enemy {
  fn default() -> Self {
    Enemy::new(Vec2::new(100., 100.))
  }
}

impl Enemy {
  pub fn new(pos: Vec2) -> Self {
    Enemy {
      pos,
      size: Vec2::new(10., 10.),
      // TODO: default path can be also empty, if enemy stands
      // TODO: this shoud be editable trough gui
      path_default: vec![pos, Vec2::new(200., 300.)],
      pov: vec![],
      // TODO: this shoud be editable trough gui
      pov_dest: Vec2::new(pos.x + VIEW_DISTANCE, pos.y),
      is_selected: false,
      path: vec![],
      res_path: String::new(),
    }
  }

  pub fn update(&mut self) {
    self.walk();

    let d = self.pov_dest - self.pos;

    // TODO: limit pov
    self.pov_dest.x = f32::cos(ONE_DEGREE) * d.x - f32::sin(ONE_DEGREE) * d.y + self.pos.x;
    self.pov_dest.y = f32::sin(ONE_DEGREE) * d.x + f32::cos(ONE_DEGREE) * d.y + self.pos.y;
  }

  pub fn draw(&self, canvas: &mut Canvas, ctx: &mut Context, state: &State) {
    if state.mode == Mode::Runtime {
      // draw texture
      if let Some(resource) = state.get_resource_by(self.res_path.clone()) {
        // calculate position manually because Image uses relative offset
        // @see https://github.com/ggez/ggez/blob/devel/docs/FAQ.md#offsets
        let pos = (self.pos - state.offset - (self.size / 2.)) * state.scale.x;
        canvas.draw(&resource.image, DrawParam::new().dest(pos).scale(state.scale));
      }

      // draw rectangle
      let color = if self.is_selected { Color::WHITE } else { Color::BLACK };
      let mesh = Mesh::new_rectangle(ctx, DrawMode::stroke(1.), self.get_rect(), color).unwrap();
      canvas.draw(&mesh, DrawParam::new().offset(state.offset).scale(state.scale));

      // draw pov
      if self.is_selected {
        let mesh =
          Mesh::new_polygon(ctx, DrawMode::fill(), &self.pov[..], Color::from_rgba(255, 0, 0, 127))
            .unwrap();
        canvas.draw(&mesh, DrawParam::new().offset(state.offset).scale(state.scale));
      }
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
    // update pov corresponding to new position
    let d = self.pos - pos;
    self.pov_dest.x -= d.x;
    self.pov_dest.y -= d.y;

    // update position
    self.pos = pos;
  }

  pub fn set_resource(&mut self, res: Resource) {
    self.res_path = res.path;
    // TODO: set_size()
    self.size.x = res.w;
    self.size.y = res.h;
  }

  fn walk(&mut self) {
    if self.path.len() > 0 {
      let next = self.path[0];
      let dist = distance::<f32, Vec2f>(next.into(), self.pos.into());

      if dist < 1. {
        self.path.remove(0);
        self.set_pos(next);
      } else {
        let d = next - self.pos;
        self.set_pos(self.pos + (d / dist));
      }
    } else {
      self.path = self.path_default.clone(); // reset path if any
    }
  }
}
