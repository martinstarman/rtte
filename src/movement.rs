use bevy::{math::CompassOctant, prelude::*};

use crate::direction::Direction;

const SPEED_WALK: f32 = 0.5;
const SPEED_RUN: f32 = 2.;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Speed {
  Slow = 1,
  Fast = 2,
}

#[derive(Clone)]
pub struct PathItem {
  pub position: Vec2,
  pub speed: Speed,
}

#[derive(Component, Default)]
pub struct Movement {
  pub path: Vec<PathItem>,
  pub default_path: Vec<PathItem>,
}

pub fn path_draw(query: Query<(&Transform, &Movement)>, mut gizmos: Gizmos) {
  for (transform, movement) in &query {
    if movement.path.len() > 0 {
      let start = PathItem {
        position: transform.translation.xy(),
        speed: movement.path[0].speed,
      };
      let mut path = vec![start];

      path.extend(movement.path.clone());

      for i in 0..path.len() - 1 {
        let (line, center) = Segment2d::from_points(path[i].position, path[i + 1].position);

        gizmos.primitive_2d(
          &line,
          Isometry2d::from_translation(center),
          Color::linear_rgba(
            1.,
            1.,
            1.,
            if movement.path[i].speed == Speed::Slow {
              0.25
            } else {
              0.5
            },
          ),
        );
      }
    }
  }
}

pub fn path_reset(mut query: Query<&mut Movement, Changed<Movement>>) {
  for mut movement in &mut query {
    if movement.path.len() == 0 && movement.default_path.len() > 0 {
      movement.path = movement.default_path.clone();
    }
  }
}

pub fn path_follow(mut query: Query<(&mut Movement, &mut Transform)>) {
  for (mut movement, mut transform) in &mut query {
    if movement.path.len() > 0 {
      let next = movement.path[0].position.extend(transform.translation.z);
      let speed = if movement.path[0].speed == Speed::Slow {
        SPEED_WALK
      } else {
        SPEED_RUN
      };

      let step = (next - transform.translation).normalize() * speed;

      transform.translation += step;

      if transform.translation.distance(next) <= speed / 2. {
        movement.path.remove(0);
      }
    }
  }
}

pub fn path_direction(
  mut query: Query<(&Movement, &mut Direction, &Transform), Changed<Movement>>,
) {
  for (movement, mut direction, transform) in &mut query {
    if movement.path.len() > 0 {
      let dir = Dir2::new(movement.path[0].position - transform.translation.xy()).unwrap();
      direction.value = CompassOctant::try_from(dir).unwrap();
    }
  }
}
