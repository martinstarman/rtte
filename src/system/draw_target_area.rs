use bevy_ecs::system::Res;
use macroquad::{color::WHITE, shapes::draw_rectangle_lines};

use crate::{
  constants::DEBUG,
  resource::{offset::Offset, target_area::TargetArea},
};

pub fn draw_target_area(target_area: Res<TargetArea>, offset: Res<Offset>) {
  if DEBUG {
    draw_rectangle_lines(
      target_area.rect.x - offset.x,
      target_area.rect.y - offset.y,
      target_area.rect.w,
      target_area.rect.h,
      1.,
      WHITE,
    );
  }
}
