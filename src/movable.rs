use bevy::{math::bounding::BoundingVolume, prelude::*};

use crate::{
  bounding_box::BoundingBox,
  direction::{Direction, Directions},
};

const SPEED_WALK: f32 = 1.;
const SPEED_RUN: f32 = 2.;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Speed {
  Slow = 1,
  Fast = 2,
}

// TODO: idle for 5s/fps?
#[derive(Clone)]
pub struct PathItem {
  pub position: Vec2,
  pub speed: Speed, //
}

#[derive(Component, Default)]
pub struct Movable {
  pub path: Vec<PathItem>,
  pub default_path: Vec<PathItem>,
}

pub fn path_draw(query: Query<(&Transform, &Movable)>, mut gizmos: Gizmos) {
  for (transform, movable) in &query {
    if movable.path.len() > 0 {
      let start = PathItem {
        position: transform.translation.xy(),
        speed: movable.path[0].speed,
      };
      let mut path = vec![start];

      path.extend(movable.path.clone());

      for i in 0..path.len() - 1 {
        let (line, center) = Segment2d::from_points(path[i].position, path[i + 1].position);

        // TODO: do not use gizmos
        gizmos.primitive_2d(
          &line,
          center,
          0.,
          Color::linear_rgba(
            1.,
            1.,
            1.,
            if movable.path[i].speed == Speed::Slow {
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

pub fn path_reset(mut query: Query<&mut Movable, Changed<Movable>>) {
  for mut movable in &mut query {
    if movable.path.len() == 0 && movable.default_path.len() > 0 {
      movable.path = movable.default_path.clone();
    }
  }
}

pub fn path_follow(mut query: Query<(&mut Movable, &mut BoundingBox, &mut Transform)>) {
  for (mut movable, mut bounding_box, mut transform) in &mut query {
    if movable.path.len() > 0 {
      let next = movable.path[0].position.extend(transform.translation.z);
      let speed = if movable.path[0].speed == Speed::Slow {
        SPEED_WALK
      } else {
        SPEED_RUN
      };

      let step = (next - transform.translation).normalize() * speed;

      transform.translation += step;
      bounding_box.value.translate_by(step.xy());

      if transform.translation.distance(next) <= speed / 2. {
        movable.path.remove(0);
      }
    }
  }
}

pub fn path_direction(mut query: Query<(&Movable, &mut Direction, &Transform), Changed<Movable>>) {
  for (movable, mut direction, transform) in &mut query {
    if movable.path.len() > 0 {
      let angle = (movable.path[0].position - transform.translation.xy()).to_angle();
      // TODO:
      // let v: Vec2  = position - translation
      // Dir2::new(v)
      // CompassOctant::from(Dir2)
      direction.value = Directions::try_from(angle).unwrap();
    }
  }
}
