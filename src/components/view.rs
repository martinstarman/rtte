use bevy_ecs::prelude::*;
use ggez::mint::Point2;

#[derive(PartialEq)]
pub enum Shift {
  LEFT = 0,
  RIGHT = 1,
}

#[derive(Component)]
pub struct View {
  pub current_direction: f32,
  pub default_direction: f32,
  pub points: Vec<Point2<f32>>,
  pub shift: Shift,
}
