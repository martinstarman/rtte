use bevy_ecs::event::Event;

#[derive(Event)]
pub struct SelectOrStopPlayer {}

impl Default for SelectOrStopPlayer {
  fn default() -> SelectOrStopPlayer {
    SelectOrStopPlayer {}
  }
}
