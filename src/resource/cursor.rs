use bevy_ecs::system::Resource;
use macroquad::{
  color::WHITE,
  input::{mouse_position, show_mouse},
  texture::{draw_texture, load_texture, Texture2D},
};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum CursorType {
  Default = 1,
  Knife = 2,
}

#[derive(Resource)]
pub struct Cursor {
  pub current_cursor: CursorType,
  // TODO: type -> texture map
  cursor_texture_default: Texture2D,
  cursor_texture_knife: Texture2D,
}

impl Cursor {
  pub async fn new() -> Self {
    show_mouse(false);

    let cursor_texture_default = load_texture("resources/cursor.default.png").await.unwrap();
    let cursor_texture_knife = load_texture("resources/cursor.knife.png").await.unwrap();

    Cursor {
      current_cursor: CursorType::Default,
      cursor_texture_default,
      cursor_texture_knife,
    }
  }

  pub fn set_type(&mut self, cursor_type: CursorType) {
    self.current_cursor = cursor_type;
  }

  pub fn draw(&self) {
    let (x, y) = mouse_position();

    draw_texture(
      if self.current_cursor == CursorType::Default {
        &self.cursor_texture_default
      } else {
        &self.cursor_texture_knife
      },
      x,
      y,
      WHITE,
    );
  }
}
