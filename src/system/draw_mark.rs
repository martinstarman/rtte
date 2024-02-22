use bevy_ecs::system::Res;
use macroquad::{color::WHITE, shapes::draw_rectangle_lines};

use crate::resource::{mark::Mark, offset::Offset};

pub fn draw_mark(view_mark: Res<Mark>, offset: Res<Offset>) {
  if let Some(position) = view_mark.position {
    draw_rectangle_lines(
      position.x - offset.x - 10.,
      position.y - offset.y - 10.,
      20.,
      20.,
      1.,
      WHITE,
    );
  }
}
