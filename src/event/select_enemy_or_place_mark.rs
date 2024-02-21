use bevy_ecs::event::Event;

#[derive(Event)]
pub struct SelectEnemyOrPlaceMark {
  pub x: f32,
  pub y: f32,
}
