use bevy_ecs::prelude::*;
use ggez::mint::Point2;

#[derive(PartialEq)]
pub enum Shift {
  LEFT = 0,
  RIGHT = 1,
}

#[derive(Component)]
pub struct View {
  pub current_direction: f32, // where is entity looking
  pub default_direction: f32, // where is entity moving
  pub polygon: Vec<Point2<f32>>,
  pub shift: Shift,
}
