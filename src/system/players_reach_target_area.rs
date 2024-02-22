use bevy_ecs::{
  query::With,
  system::{Query, ResMut},
};
use macroquad::math::Vec2;

use crate::{
  component::{player::PlayerComponent, position::PositionComponent},
  resource::target_area::TargetArea,
};

pub fn players_reach_target_area(
  query: Query<&PositionComponent, With<PlayerComponent>>,
  target_area: ResMut<TargetArea>,
) {
  let mut all_players_in_target_area = true;

  for position in &query {
    if !target_area.rect.contains(Vec2::new(position.x, position.y)) {
      all_players_in_target_area = false;
    }
  }

  if all_players_in_target_area {
    // TODO: do something
  }
}
