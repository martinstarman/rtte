use bevy_ecs::system::Res;
use macroquad::{color::WHITE, shapes::draw_rectangle_lines};

use crate::resource::{view_mark::ViewMark, offset::Offset};

pub fn draw_mark(view_mark: Res<ViewMark>, offset: Res<Offset>) {
  if let Some(position) = view_mark.position {
    // TODO: draw some texture
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
