use bevy_ecs::prelude::*;

#[derive(Event)]
pub struct SelectEnemyOrPlaceMark {
  pub x: f32,
  pub y: f32,
}
