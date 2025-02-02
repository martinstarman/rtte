use bevy::prelude::*;

use crate::direction::{Direction, Directions};

const MOVABLE_SPEED_WALK: f32 = 0.5;
const MOVABLE_SPEED_RUN: f32 = 2.;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MovableSpeed {
  Walk = 1,
  Run = 2,
}

#[derive(Clone, Copy)]
pub struct MovablePathItem {
  pub position: Vec2,
  pub speed: MovableSpeed,
  pub wait_frame_count: u8,
}

#[derive(Component, Default)]
pub struct Movable {
  pub path: Vec<MovablePathItem>,
  pub default_path: Vec<MovablePathItem>,
}

pub fn path_draw(query: Query<(&Transform, &Movable)>, mut gizmos: Gizmos) {
  for (transform, movable) in &query {
    if movable.path.len() > 0 {
      let start = transform.translation.xy();
      let mut path = vec![start];

      path.extend(
        movable
          .path
          .iter()
          .map(|item| item.position.xy())
          .collect::<Vec<_>>(),
      );

      for i in 0..path.len() - 1 {
        let (line, center) = Segment2d::from_points(path[i], path[i + 1]);

        // TODO: stop using gizmos
        gizmos.primitive_2d(
          &line,
          Isometry2d::from_translation(center),
          Color::linear_rgba(1., 1., 1., 0.5),
        );
      }
    }
  }
}

pub fn path_reset(mut query: Query<&mut Movable, Changed<Movable>>) {
  for mut movable in &mut query {
    if movable.path.len() == 0 && movable.default_path.len() > 0 {
      movable.path = movable.default_path.clone();
    }
  }
}

// TODO: line of sight is looking to wrong target when enemy is waiting
pub fn path_follow(mut query: Query<(&mut Movable, &mut Transform)>) {
  for (mut movable, mut transform) in &mut query {
    if movable.path.len() > 0 {
      let curr_pos = transform.translation.xy();
      let next_pos = movable.path[0].position;
      let speed = if movable.path[0].speed == MovableSpeed::Walk {
        MOVABLE_SPEED_WALK
      } else {
        MOVABLE_SPEED_RUN
      };

      if curr_pos.distance(next_pos) <= speed / 2. {
        if movable.path[0].wait_frame_count > 0 {
          movable.path[0].wait_frame_count -= 1;
        } else {
          movable.path.remove(0);
        }
      } else {
        let step = (next_pos - curr_pos).normalize() * speed;
        transform.translation += step.extend(0.);
      }
    }
  }
}

pub fn path_direction(mut query: Query<(&Movable, &mut Direction, &Transform), Changed<Movable>>) {
  for (movable, mut direction, transform) in &mut query {
    if movable.path.len() > 0 {
      let angle = (movable.path[0].position - transform.translation.xy()).to_angle();
      direction.value = Directions::try_from(angle).unwrap();
    }
  }
}
