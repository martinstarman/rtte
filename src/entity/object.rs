use crate::{geometry::vec2::Vec2, resource::Resource, State};

use ggez::{
  graphics::{Canvas, Color, DrawMode, DrawParam, Mesh, Rect},
  Context,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Object {
  pub pos: Vec2,
  pub size: Vec2,
  #[serde(skip)]
  pub is_selected: bool,
  pub res_path: String,
}

impl Default for Object {
  fn default() -> Self {
    Object::new(Vec2::new(100., 100.), Vec2::new(50., 50.))
  }
}

impl Object {
  pub fn new(pos: Vec2, size: Vec2) -> Self {
    Object {
      pos,
      size,
      is_selected: false,
      res_path: String::new(),
    }
  }

  pub fn update(&mut self) {}

  pub fn draw(&self, canvas: &mut Canvas, ctx: &mut Context, state: &State) {
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
  }

  pub fn get_rect(&self) -> Rect {
    Rect::new(
      self.pos.x - (self.size.x / 2.),
      self.pos.y - (self.size.y / 2.),
      self.size.x,
      self.size.y,
    )
  }

  pub fn set_resource(&mut self, res: Resource) {
    self.res_path = res.path;
    // TODO: set_size()
    self.size.x = res.w;
    self.size.y = res.h;
  }

  pub fn set_pos(&mut self, pos: Vec2) {
    self.pos = pos;
  }

  pub fn set_size(&mut self, size: Vec2) {
    self.size = size;
  }
}
