use crate::{
  component::{
    field_of_view::FieldOfViewComponent,
    polygon::{PolygonComponent, Type},
    position::PositionComponent,
  },
  constants::{FIELD_OF_VIEW_DISTANCE, FIELD_OF_VIEW_INNER_ANGLE, RADIAN},
};
use bevy_ecs::system::Query;
use macroquad::math::Vec2;
use maths_rs::{Vec2f, Vec3f};

pub fn field_of_view(
  mut query1: Query<(&mut FieldOfViewComponent, &PositionComponent)>,
  query2: Query<&PolygonComponent>,
) {
  let polygons: Vec<&PolygonComponent> =
    query2.iter().filter(|polygon| polygon.r#type == Type::BLOCK).collect();

  for (mut field_of_view, position) in &mut query1 {
    let mut points: Vec<Vec2> = vec![];
    let mut angle = field_of_view.direction - (FIELD_OF_VIEW_INNER_ANGLE / 2.);

    while angle < field_of_view.direction + (FIELD_OF_VIEW_INNER_ANGLE / 2.) {
      let mut min_distance = FIELD_OF_VIEW_DISTANCE;
      let mut point = Vec2f::new(
        f32::cos(angle) * FIELD_OF_VIEW_DISTANCE + position.x,
        f32::sin(angle) * FIELD_OF_VIEW_DISTANCE + position.y,
      );

      for polygon in &polygons {
        // test all polygon lines vs ray (from entity position to fov point)
        for line in &polygon.lines {
          if let Some(intersection) = maths_rs::line_segment_vs_line_segment(
            Vec3f::new(position.x, position.y, 0.),
            point.into(),
            Vec3f::new(line.0.x, line.0.y, 0.),
            Vec3f::new(line.1.x, line.1.y, 0.),
          ) {
            // ray was intersected by some line
            let distance = maths_rs::distance::<f32, Vec2f>(
              Vec2f::new(position.x, position.y),
              intersection.into(),
            );

            // save the point if the intersection is closer to entity
            if distance < min_distance {
              point = intersection.into();
              min_distance = distance;
            }
          }
        }
      }

      // add closest point to entity
      points.push(Vec2::new(point.x, point.y));

      angle += RADIAN;
    }

    // close fov polygon
    points.push(Vec2::new(position.x, position.y));

    field_of_view.points = points;
  }
}