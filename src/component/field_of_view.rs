use bevy_ecs::component::Component;
use macroquad::math::Vec2;

#[derive(PartialEq)]
pub enum Shift {
  LEFT = 0,
  RIGHT = 1,
}

#[derive(Component)]
pub struct FieldOfViewComponent {
  pub direction: f32,
  pub movement_direction: f32,
  pub points: Vec<Vec2>,
  pub shift: Shift,
}
