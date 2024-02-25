use bevy_ecs::component::Component;
use maths_rs::vec::Vec2;

#[derive(PartialEq)]
pub enum Shift {
  LEFT = 0,
  RIGHT = 1,
}

#[derive(Component)]
pub struct FieldOfViewComponent {
  pub direction: f32,
  pub movement_direction: f32,
  pub points: Vec<Vec2<f32>>,
  pub shift: Shift,
}
