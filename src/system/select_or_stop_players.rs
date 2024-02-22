use crate::{
  component::{
    movement::MovementComponent, player::PlayerComponent, selection::SelectionComponent,
  },
  event::select_or_stop_player::SelectOrStopPlayer,
};
use bevy_ecs::{event::EventReader, query::With, system::Query};

pub fn select_or_stop_players(
  mut events: EventReader<SelectOrStopPlayer>,
  mut query: Query<(&mut MovementComponent, &SelectionComponent), With<PlayerComponent>>,
) {
  for _event in events.read() {
    // stop selected player movement
    for (mut movement, selection) in &mut query {
      if selection.active {
        movement.current_path = vec![];
      }
    }
  }
}
