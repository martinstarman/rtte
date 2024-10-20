use bevy::prelude::*;

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
pub struct Movable {
  pub path: Vec<PathItem>,
}

pub fn path_draw(query: Query<(&Transform, &Movable)>, mut gizmos: Gizmos) {
  for (transform, movable) in &query {
    if movable.path.len() > 0 {
      let start = PathItem {
        position: Vec2::new(transform.translation.x, transform.translation.y),
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
