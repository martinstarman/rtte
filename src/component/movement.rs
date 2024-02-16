use bevy_ecs::prelude::*;
use macroquad::math::Vec2;

#[derive(Component)]
pub struct MovementComponent {
  pub current_path: Vec<Vec2>,
  pub default_path: Vec<Vec2>,
}
