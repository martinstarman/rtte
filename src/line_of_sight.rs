use bevy::{math::bounding::*, prelude::*};
use core::f32;
use vleue_navigator::prelude::PrimitiveObstacle;

use crate::{
  enemy::{EnemyState, EnemyStates},
  movement::Movement,
  selection::Selection,
};

const LINE_OF_SIGHT_DISTANCE: i32 = 150;
const LINE_OF_SIGHT_INNER_ANGLE: i32 = 60;
pub const LINE_OF_SIGHT_VERTICES: usize = LINE_OF_SIGHT_INNER_ANGLE as usize + 1;

#[derive(Component)]
pub struct LineOfSightObstacle;

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

pub fn line_of_sight_update_polygon_points(
  mut query: Query<(&mut LineOfSight, &Transform, &EnemyState)>,
  obstacles: Query<(&PrimitiveObstacle, &GlobalTransform), With<LineOfSightObstacle>>,
) {
  for (mut line_of_sight, transform, enemy_state) in &mut query {
    if enemy_state.value == EnemyStates::Dead {
      return;
    }

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
      let ray = Ray2d::new(position, Dir2::new(point - position).unwrap());
      let ray_cast = RayCast2d::from_ray(ray, LINE_OF_SIGHT_DISTANCE as f32);

      points[i as usize + 1] = point;

      for (primitive_obstacle, global_transform) in &obstacles {
        match primitive_obstacle {
          PrimitiveObstacle::Rectangle(primitive) => {
            if let Some(toi) = ray_cast.aabb_intersection_at(&primitive.aabb_2d(
              Isometry2d::from_translation(global_transform.translation().xy()),
            )) {
              let intersection = ray_cast.ray.origin + *ray_cast.ray.direction * toi;

              if position.distance(intersection) < position.distance(points[i as usize + 1]) {
                points[i as usize + 1] = intersection;
              }
            }
          }
          _ => panic!("Use rectangle"),
        }
      }
    }

    line_of_sight.polygon = Polygon::new(points);
  }
}

pub fn line_of_sight_update_shift(mut query: Query<(&mut LineOfSight, &EnemyState)>) {
  for (mut line_of_sight, enemy_state) in &mut query {
    if enemy_state.value == EnemyStates::Dead {
      return;
    }

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

pub fn line_of_sight_update_looking_at_position(
  mut query: Query<(&mut LineOfSight, &Movement, &Transform), Changed<Movement>>,
) {
  for (mut line_of_sight, movement, transform) in &mut query {
    if movement.path.len() > 0 {
      line_of_sight.looking_at =
        (movement.path[0].position - transform.translation.xy()).normalize();
    }
  }
}

pub fn line_of_sight_draw_looking_at_position(
  query: Query<(&LineOfSight, &Transform, &Selection)>,
  mut gizmos: Gizmos,
) {
  for (line_of_sight, transform, selection) in &query {
    if selection.active {
      let rect = Rectangle::new(10., 10.);
      let position = transform.translation.xy();
      let looking_at = position + line_of_sight.looking_at * LINE_OF_SIGHT_DISTANCE as f32;

      gizmos.primitive_2d(
        &rect,
        Isometry2d::from_translation(looking_at),
        Color::WHITE,
      );
    }
  }
}

pub fn line_of_sight_draw_polygon(query: Query<(&LineOfSight, &Selection)>, mut gizmos: Gizmos) {
  for (line_of_sight, selection) in &query {
    if selection.active {
      gizmos.primitive_2d(
        &line_of_sight.polygon,
        Isometry2d::from_translation(Vec2::ZERO),
        Color::WHITE,
      );
    }
  }
}
