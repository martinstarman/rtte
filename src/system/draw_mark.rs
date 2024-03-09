use bevy_ecs::system::Res;
use macroquad::{color::WHITE, texture::draw_texture};

use crate::resource::{mark::Mark, offset::Offset};

pub fn draw_mark(mark: Res<Mark>, offset: Res<Offset>) {
  if let Some(position) = mark.position {
    draw_texture(&mark.texture, position.x - offset.x - 10., position.y - offset.y - 10., WHITE);
  }
}
