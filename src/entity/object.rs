use crate::{geometry::vec2::Vec2, State};

use ggez::{
  graphics::{Canvas, Color, DrawMode, DrawParam, Image, Mesh, Rect, Drawable},
  Context,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Object {
  pub pos: Vec2,
  pub size: Vec2,
  pub texture_path: String,
  #[serde(skip)]
  pub is_selected: bool,
  #[serde(skip)]
  texture: Option<Image>,
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
      texture_path: String::new(),
      texture: None,
    }
  }

  pub fn update(&mut self, ctx: &mut Context) {
    // TODO: remove from update()
    if self.texture_path != "" && self.texture.is_none() {
      self.texture = Some(Image::from_path(ctx, self.texture_path.clone()).unwrap());

      if let Some(dim) = self.texture.as_ref().unwrap().dimensions(ctx) {
        // TODO: set_size()
        self.size.x = dim.w;
        self.size.y = dim.h;
      }
    }
  }

  pub fn draw(&self, canvas: &mut Canvas, ctx: &mut Context, state: &State) {
    // draw texture
    if self.texture.is_some() {
      // calculate position manually because Image uses relative offset
      // @see https://github.com/ggez/ggez/blob/devel/docs/FAQ.md#offsets
      let pos = (self.pos - state.offset - (self.size / 2.)) * state.scale.x;
      canvas.draw(self.texture.as_ref().unwrap(), DrawParam::new().dest(pos).scale(state.scale));
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

  pub fn set_texture_path(&mut self, path: String) {
    self.texture_path = path;
    self.texture = None;
  }

  pub fn set_pos(&mut self, pos: Vec2) {
    self.pos = pos;
  }

  pub fn set_size(&mut self, size: Vec2) {
    self.size = size;
  }
}
