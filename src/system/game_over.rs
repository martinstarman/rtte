use bevy_ecs::{
  query::With,
  system::{Query, ResMut},
};
use ggez::mint::Point2;
use maths_rs::vec::Vec2;

use crate::{
  component::{player::PlayerComponent, position::PositionComponent, view::ViewComponent},
  resource::target_area::TargetArea,
};

pub fn all_players_in_target_area(
  query: Query<&PositionComponent, With<PlayerComponent>>,
  target_area: ResMut<TargetArea>,
) {
  let mut all_players_in_target_area = true;

  for position in &query {
    if !target_area.rect.contains(Point2 {
      x: position.x,
      y: position.y,
    }) {
      all_players_in_target_area = false;
    }
  }

  if all_players_in_target_area {
    // TODO: do something
  }
}

pub fn some_player_in_enemy_view(
  query: Query<&ViewComponent>,
  query2: Query<&PositionComponent, With<PlayerComponent>>,
) {
  for view in &query {
    for position in &query2 {
      if view.polygon.len() > 2
        && maths_rs::point_inside_polygon(
          Vec2 {
            x: position.x,
            y: position.y,
          },
          &view.polygon.iter().map(|p| Vec2::new(p.x, p.y)).collect::<Vec<Vec2<f32>>>(),
        )
      {
        // TODO: do something
      }
    }
  }
}
