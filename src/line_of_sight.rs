use bevy::{math::bounding::*, prelude::*};
use core::f32;

use crate::{bounding_box::BoundingBox, movable::Movable, obstacle::Obstacle};

const LINE_OF_SIGHT_DISTANCE: i32 = 150;
const LINE_OF_SIGHT_INNER_ANGLE: i32 = 60;
pub const LINE_OF_SIGHT_VERTICES: usize = LINE_OF_SIGHT_INNER_ANGLE as usize + 1;

#[derive(Component)]
pub struct LineOfSight {
  /// where is line of sight looking (center point)
  ///
  /// ---o---
  ///  \   /
  ///   \ /
  ///    V
  ///
  pub looking_at: Vec2,

  /// offset from center point in degrees (in range <-INNER_ANGLE/2;+INNER_ANGLE/2>)
  pub offset: i32,

  /// current line of sight shift
  pub shift: LineOfSightShift,

  /// current line of sight polygon
  pub polygon: Polygon<LINE_OF_SIGHT_VERTICES>,
}

#[derive(Component, PartialEq, Eq)]
pub enum LineOfSightShift {
  Left = 0,
  Right = 1,
}

pub fn line_of_sight_update(
  mut query: Query<(&mut LineOfSight, &Transform)>,
  obstacles: Query<&BoundingBox, With<Obstacle>>,
) {
  for (mut line_of_sight, transform) in &mut query {
    let position = transform.translation.xy();
    let looking_at = position + line_of_sight.looking_at * LINE_OF_SIGHT_DISTANCE as f32;
    let mut points = [Vec2::ZERO; LINE_OF_SIGHT_VERTICES];
    points[0] = position;

    let mut point_transform = Transform::from_translation(looking_at.extend(0.));

    // get the first point on "left" side
    //
    // o------
    //  \   /
    //   \ /
    //    V
    //
    point_transform.rotate_around(
      transform.translation,
      Quat::from_axis_angle(
        Vec3::Z,
        ((line_of_sight.offset - LINE_OF_SIGHT_INNER_ANGLE / 2) as f32).to_radians(),
      ),
    );

    for i in 0..LINE_OF_SIGHT_INNER_ANGLE {
      // get next point by rotating by 1 degree
      point_transform.rotate_around(
        transform.translation,
        Quat::from_axis_angle(Vec3::Z, (1 as f32).to_radians()),
      );

      let point = point_transform.translation.xy();
      let ray = Ray2d::new(position, (point - position).normalize());
      let ray_cast = RayCast2d::from_ray(ray, LINE_OF_SIGHT_DISTANCE as f32);

      points[i as usize + 1] = point;

      for bounding_box in &obstacles {
        if let Some(toi) = ray_cast.aabb_intersection_at(&bounding_box.value) {
          let intersection = ray_cast.ray.origin + *ray_cast.ray.direction * toi;

          if position.distance(intersection) < position.distance(points[i as usize + 1]) {
            points[i as usize + 1] = intersection;
          }
        }
      }
    }

    line_of_sight.polygon = Polygon::new(points);
  }
}

pub fn line_of_sight_shift(mut query: Query<&mut LineOfSight>) {
  for mut line_of_sight in &mut query {
    line_of_sight.offset += if line_of_sight.shift == LineOfSightShift::Left {
      1
    } else {
      -1
    };

    if line_of_sight.offset >= LINE_OF_SIGHT_INNER_ANGLE / 2 {
      line_of_sight.shift = LineOfSightShift::Right;
    }

    if line_of_sight.offset <= -LINE_OF_SIGHT_INNER_ANGLE / 2 {
      line_of_sight.shift = LineOfSightShift::Left;
    }
  }
}

pub fn line_of_sight_looking_at(
  mut query: Query<(&mut LineOfSight, &Movable, &Transform), Changed<Movable>>,
) {
  for (mut line_of_sight, movable, transform) in &mut query {
    if movable.path.len() > 0 {
      line_of_sight.looking_at =
        (movable.path[0].position - transform.translation.xy()).normalize();
    }
  }
}

pub fn line_of_sight_looking_at_draw(query: Query<(&LineOfSight, &Transform)>, mut gizmos: Gizmos) {
  for (line_of_sight, transform) in &query {
    let rect = Rectangle::new(10., 10.);
    let position = transform.translation.xy();
    let looking_at = position + line_of_sight.looking_at * LINE_OF_SIGHT_DISTANCE as f32;

    // TODO: stop using gizmos
    gizmos.primitive_2d(&rect, looking_at, 0., Color::WHITE);
  }
}

pub fn line_of_sight_draw(query: Query<&LineOfSight>, mut gizmos: Gizmos) {
  for line_of_sight in &query {
    // TODO: stop using gizmos
    gizmos.primitive_2d(&line_of_sight.polygon, Vec2::ZERO, 0., Color::WHITE);
  }
}
