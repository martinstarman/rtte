use bevy_ecs::prelude::*;

#[derive(Event)]
pub struct SelectOrMovePlayer {
  pub x: f32,
  pub y: f32,
}
