use bevy_ecs::{
  query::With,
  system::{Query, ResMut},
};
use ggez::mint::Point2;

use crate::{
  component::{player::PlayerComponent, position::PositionComponent},
  resource::target_area::TargetArea,
};

pub fn run(
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
