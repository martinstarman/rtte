use crate::{
  component::{position::PositionComponent, shape::ShapeComponent, size::SizeComponent},
  resource::offset::Offset,
};
use bevy_ecs::system::{Query, Res};
use macroquad::{
  color::WHITE,
  shapes::{draw_line, draw_rectangle_lines},
};

pub fn draw_entity_debug(
  query1: Query<(&PositionComponent, &SizeComponent)>,
  query2: Query<(&PositionComponent, &ShapeComponent)>,
  offset: Res<Offset>,
) {
  // rect
  for (position, size) in &query1 {
    draw_rectangle_lines(
      position.x - offset.x,
      position.y - offset.y,
      size.width,
      size.height,
      1.,
      WHITE,
    );
  }

  // shape
  for (position, shape) in &query2 {
    for line in &shape.lines {
      draw_line(
        position.x + line.0.x - offset.x,
        position.y + line.0.y - offset.y,
        position.x + line.1.x - offset.x,
        position.y + line.1.y - offset.y,
        1.0,
        WHITE,
      );
    }
  }
}
