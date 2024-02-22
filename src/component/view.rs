use bevy_ecs::component::Component;
use macroquad::math::Vec2;

#[derive(PartialEq)]
pub enum Shift {
  LEFT = 0,
  RIGHT = 1,
}

#[derive(Component)]
pub struct ViewComponent {
  pub current_direction: f32, // TODO: where is entity looking
  pub default_direction: f32, // TODO: where is entity moving
  pub points: Vec<Vec2>,
  pub shift: Shift,
}
