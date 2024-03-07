use bevy_ecs::{
  query::With,
  system::{Query, ResMut},
};
use maths_rs::vec::Vec2;

use crate::{
  component::{
    field_of_view::FieldOfViewComponent, player::PlayerComponent, position::PositionComponent,
  },
  resource::alarm::Alarm,
};

pub fn some_player_in_enemy_field_of_view(
  query: Query<&FieldOfViewComponent>,
  query2: Query<&PositionComponent, With<PlayerComponent>>,
  mut alarm: ResMut<Alarm>,
) {
  for field_of_view in &query {
    for position in &query2 {
      if field_of_view.points.len() > 2
        && maths_rs::point_inside_polygon(
          Vec2 {
            x: position.x,
            y: position.y,
          },
          &field_of_view.points,
        )
      {
        alarm.set_active();
        // TODO: mission failed
      }
    }
  }
}
