use bevy::prelude::*;

use crate::player::{Player, PlayerStates};

#[derive(Clone)]
pub struct PathItem<T> {
  pub position: Vec2,
  pub state: T,
}

#[derive(Component, Default)]
pub struct Movable<T> {
  pub path: Vec<PathItem<T>>,
}

pub fn draw_player_path(
  query: Query<(&Transform, &Movable<PlayerStates>), With<Player>>,
  mut gizmos: Gizmos,
) {
  for (transform, movable) in &query {
    if movable.path.len() > 0 {
      let start: PathItem<PlayerStates> = PathItem {
        position: Vec2::new(transform.translation.x, transform.translation.y),
        state: movable.path[0].state,
      };
      let mut path: Vec<PathItem<PlayerStates>> = vec![start];
      path.extend(movable.path.clone());

      for i in 0..path.len() - 1 {
        let (line, center) = Segment2d::from_points(path[i].position, path[i + 1].position);

        // TODO: do not use gizmos when bevy has support for drawing segment primitive
        gizmos.primitive_2d(
          &line,
          center,
          0.,
          Color::linear_rgba(
            1.,
            1.,
            1.,
            if movable.path[i].state == PlayerStates::Walk {
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
