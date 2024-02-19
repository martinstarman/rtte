use bevy_ecs::event::Event;

#[derive(Event)]
pub struct SelectOrMovePlayer {
  pub x: f32,
  pub y: f32,
}
