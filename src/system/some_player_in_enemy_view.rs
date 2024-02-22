use bevy_ecs::{query::With, system::Query};
use maths_rs::vec::Vec2;

use crate::component::{player::PlayerComponent, position::PositionComponent, view::ViewComponent};

pub fn some_player_in_enemy_view(query: Query<&ViewComponent>, query2: Query<&PositionComponent, With<PlayerComponent>>) {
  for view in &query {
    for position in &query2 {
      if view.points.len() > 2
        && maths_rs::point_inside_polygon(
          Vec2 {
            x: position.x,
            y: position.y,
          },
          &view.points.iter().map(|p| Vec2::new(p.x, p.y)).collect::<Vec<Vec2<f32>>>(),
        )
      {
        // TODO: do something
      }
    }
  }
}
