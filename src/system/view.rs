use crate::{
  component::{
    object::{ObjectComponent, PolygonType},
    position::PositionComponent,
    view::ViewComponent,
  },
  constants::{RADIAN, VIEW_DISTANCE, VIEW_INNER_ANGLE},
};
use bevy_ecs::system::Query;
use ggez::mint::Point2;
use maths_rs::{Vec2f, Vec3f};

pub fn run(
  mut query: Query<(&mut ViewComponent, &PositionComponent)>,
  query2: Query<&ObjectComponent>,
) {
  let objects: Vec<&ObjectComponent> =
    query2.iter().filter(|object| object.polygon_type == PolygonType::BLOCK).collect();

  for (mut view, position) in &mut query {
    let mut points: Vec<Point2<f32>> = vec![];
    let mut rad = view.current_direction - (VIEW_INNER_ANGLE / 2.);

    while rad < view.current_direction + (VIEW_INNER_ANGLE / 2.) {
      let mut min_distance = VIEW_DISTANCE;
      let mut point = Vec2f::new(
        f32::cos(rad) * VIEW_DISTANCE + position.x,
        f32::sin(rad) * VIEW_DISTANCE + position.y,
      );

      for object in &objects {
        // test all objects polygon lines vs ray (from entity position to view point)
        for line in &object.polygon {
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
      points.push(Point2 {
        x: point.x,
        y: point.y,
      });

      rad += RADIAN;
    }

    // close view polygon
    points.push(Point2 {
      x: position.x,
      y: position.y,
    });

    view.polygon = points;
  }
}
