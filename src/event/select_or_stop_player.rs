use bevy_ecs::prelude::*;

#[derive(Event)]
pub struct SelectOrStopPlayer {}

impl Default for SelectOrStopPlayer {
  fn default() -> SelectOrStopPlayer {
    SelectOrStopPlayer {}
  }
}
