use bevy::prelude::*;
use core::f32;
use vleue_navigator::prelude::PrimitiveObstacle;

use crate::{
  enemy::{EnemyState, EnemyStates},
  movement::Movement,
  selection::Selection,
};

const CONE_OF_VIEW_DISTANCE: i32 = 150;
const CONE_OF_VIEW_INNER_ANGLE: i32 = 60;
pub const CONE_OF_VIEW_VERTICES: usize = CONE_OF_VIEW_INNER_ANGLE as usize * 3;

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

  /// mesh handle
  pub mesh_handle: Handle<Mesh>,
}

#[derive(Component, PartialEq, Eq)]
pub enum ConeOfViewShift {
  Left = 0,
  Right = 1,
}

pub fn cone_of_view_update_mesh(
  query: Query<(&ConeOfView, &Transform, &EnemyState)>,
  obstacles: Query<(&PrimitiveObstacle, &GlobalTransform), With<ConeOfViewObstacle>>,
  mut meshes: ResMut<Assets<Mesh>>,
) {
  for (cone_of_view, transform, enemy_state) in &query {
    if enemy_state.value == EnemyStates::Dead {
      continue;
    }

    let position = transform.translation.xy();
    let looking_at = position + cone_of_view.looking_at * CONE_OF_VIEW_DISTANCE as f32;
    let mut points = [Vec2::ZERO; CONE_OF_VIEW_INNER_ANGLE as usize];
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
        Quat::from_axis_angle(Vec3::Z, 1.0_f32.to_radians()),
      );

      let point = point_transform.translation.xy();
      points[i as usize] = point;

      for (primitive_obstacle, global_transform) in &obstacles {
        match primitive_obstacle {
          PrimitiveObstacle::ConvexPolygon(polygon) => {
            for j in 0..polygon.vertices().len() {
              let p = polygon.vertices()[j] + global_transform.translation().xy();
              let q = polygon.vertices()[if j + 1 >= polygon.vertices().len() {
                0
              } else {
                j + 1
              }] + global_transform.translation().xy();

              if let Some(intersection) = maths_rs::line_segment_vs_line_segment(
                maths_rs::vec::Vec3::new(position.x, position.y, 0.),
                maths_rs::vec::Vec3::new(point.x, point.y, 0.),
                maths_rs::vec::Vec3::new(p.x, p.y, 0.),
                maths_rs::vec::Vec3::new(q.x, q.y, 0.),
              ) {
                let vec = Vec2::new(intersection.x, intersection.y);
                if position.distance(vec) < position.distance(points[i as usize]) {
                  points[i as usize] = vec;
                }
              }
            }
          }
          _ => panic!("Convex polygon expected"),
        }
      }
    }

    let mesh = meshes.get_mut(&cone_of_view.mesh_handle).unwrap();
    let mesh_positions = mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION).unwrap();
    let mut triangles = vec![];

    for j in 0..CONE_OF_VIEW_INNER_ANGLE - 1 {
      triangles.push(position.extend(0.0) - transform.translation);
      triangles.push(points[j as usize].extend(0.0) - transform.translation);
      triangles.push(points[j as usize + 1].extend(0.0) - transform.translation);
    }

    *mesh_positions = triangles.to_vec().into();
  }
}

pub fn cone_of_view_toggle_shift(mut query: Query<(&mut ConeOfView, &EnemyState)>) {
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

pub fn cone_of_view_toggle_visibility(
  mut query: Query<(&Selection, &Children), Changed<Selection>>,
  mut visibility: Query<&mut Visibility>,
) {
  for (selection, children) in &mut query {
    for child in children.iter() {
      if let Ok(mut cone_of_view_visibility) = visibility.get_mut(child) {
        *cone_of_view_visibility = if selection.active {
          Visibility::Visible
        } else {
          Visibility::Hidden
        }
      };
    }
  }
}
