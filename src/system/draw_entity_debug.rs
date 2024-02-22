use crate::{
  component::{polygon::PolygonComponent, position::PositionComponent, size::SizeComponent},
  resource::offset::Offset,
};
use bevy_ecs::system::{Query, Res};
use macroquad::{
  color::WHITE,
  shapes::{draw_line, draw_rectangle_lines},
};

pub fn run(
  query1: Query<(&PositionComponent, &SizeComponent)>,
  query2: Query<&PolygonComponent>,
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

  // polygon
  for object in &query2 {
    if object.polygon.len() >= 3 {
      for line in &object.polygon {
        draw_line(
          line.0.x - offset.x,
          line.0.y - offset.y,
          line.1.x - offset.x,
          line.1.y - offset.y,
          1.0,
          WHITE,
        );
      }
    }
  }
}
