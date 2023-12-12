use crate::{
  component::{
    movement::MovementComponent, player::PlayerComponent, selection::SelectionComponent,
  },
  event::select_or_stop_player::SelectOrStopPlayer,
};
use bevy_ecs::{event::EventReader, system::Query};

pub fn run(
  mut events: EventReader<SelectOrStopPlayer>,
  mut query: Query<(&PlayerComponent, &mut MovementComponent, &SelectionComponent)>,
) {
  for _event in events.read() {
    // TODO: multiple player selection

    // stop selected player movement
    for (_, mut movement, selection) in &mut query {
      if selection.active {
        movement.current_path = vec![];
      }
    }
  }
}
