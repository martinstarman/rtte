use bevy::{math::bounding::*, prelude::*};
use core::f32;
use vleue_navigator::prelude::PrimitiveObstacle;

use crate::{
  enemy::{EnemyState, EnemyStates},
  movement::Movement,
  selection::Selection,
};

const CONE_OF_VIEW_DISTANCE: i32 = 150;
const CONE_OF_VIEW_INNER_ANGLE: i32 = 60;
pub const CONE_OF_VIEW_VERTICES: usize = CONE_OF_VIEW_INNER_ANGLE as usize + 1;

#[derive(Component)]
pub struct ConeOfViewObstacle;

#[derive(Component)]
pub struct ConeOfView {
  /// where is cone of view looking (center point)
  ///
  /// ---o---
  ///  \   /
  ///   \ /
  ///    V
  ///
  pub looking_at: Vec2,

  /// offset from center point in degrees (in range <-INNER_ANGLE/2;+INNER_ANGLE/2>)
  pub offset: i32,

  /// current cone of view shift
  pub shift: ConeOfViewShift,

  /// current cone of view polygon
  pub polygon: Polygon,
}

#[derive(Component, PartialEq, Eq)]
pub enum ConeOfViewShift {
  Left = 0,
  Right = 1,
}

pub fn cone_of_view_update_polygon_points(
  mut query: Query<(&mut ConeOfView, &Transform, &EnemyState)>,
  obstacles: Query<(&PrimitiveObstacle, &GlobalTransform), With<ConeOfViewObstacle>>,
) {
  for (mut cone_of_view, transform, enemy_state) in &mut query {
    if enemy_state.value == EnemyStates::Dead {
      continue;
    }

    let position = transform.translation.xy();
    let looking_at = position + cone_of_view.looking_at * CONE_OF_VIEW_DISTANCE as f32;
    let mut points = [Vec2::ZERO; CONE_OF_VIEW_VERTICES];
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
        ((cone_of_view.offset - CONE_OF_VIEW_INNER_ANGLE / 2) as f32).to_radians(),
      ),
    );

    for i in 0..CONE_OF_VIEW_INNER_ANGLE {
      // get next point by rotating by 1 degree
      point_transform.rotate_around(
        transform.translation,
        Quat::from_axis_angle(Vec3::Z, (1 as f32).to_radians()),
      );

      let point = point_transform.translation.xy();
      let ray = Ray2d::new(position, Dir2::new(point - position).unwrap());
      let ray_cast = RayCast2d::from_ray(ray, CONE_OF_VIEW_DISTANCE as f32);

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

    cone_of_view.polygon = Polygon::new(points);
  }
}

pub fn cone_of_view_update_shift(mut query: Query<(&mut ConeOfView, &EnemyState)>) {
  for (mut cone_of_view, enemy_state) in &mut query {
    if enemy_state.value == EnemyStates::Dead {
      continue;
    }

    cone_of_view.offset += if cone_of_view.shift == ConeOfViewShift::Left {
      1
    } else {
      -1
    };

    if cone_of_view.offset >= CONE_OF_VIEW_INNER_ANGLE / 2 {
      cone_of_view.shift = ConeOfViewShift::Right;
    }

    if cone_of_view.offset <= -CONE_OF_VIEW_INNER_ANGLE / 2 {
      cone_of_view.shift = ConeOfViewShift::Left;
    }
  }
}

pub fn cone_of_view_update_looking_at_position(
  mut query: Query<(&mut ConeOfView, &Movement, &Transform), Changed<Movement>>,
) {
  for (mut cone_of_view, movement, transform) in &mut query {
    if movement.path.len() > 0 {
      cone_of_view.looking_at =
        (movement.path[0].position - transform.translation.xy()).normalize();
    }
  }
}

pub fn cone_of_view_draw_looking_at_position(
  query: Query<(&ConeOfView, &Transform, &Selection)>,
  mut gizmos: Gizmos,
) {
  for (cone_of_view, transform, selection) in &query {
    if selection.active {
      let rect = Rectangle::new(10., 10.);
      let position = transform.translation.xy();
      let looking_at = position + cone_of_view.looking_at * CONE_OF_VIEW_DISTANCE as f32;

      gizmos.primitive_2d(
        &rect,
        Isometry2d::from_translation(looking_at),
        Color::WHITE,
      );
    }
  }
}

pub fn cone_of_view_draw_polygon(query: Query<(&ConeOfView, &Selection)>, mut gizmos: Gizmos) {
  for (cone_of_view, selection) in &query {
    if selection.active {
      gizmos.primitive_2d(
        &cone_of_view.polygon,
        Isometry2d::from_translation(Vec2::ZERO),
        Color::WHITE,
      );
    }
  }
}
