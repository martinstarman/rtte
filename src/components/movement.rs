use bevy_ecs::prelude::*;
use ggez::mint::Point2;

#[derive(Component)]
pub struct Movement {
  pub current_path: Vec<Point2<f32>>,
  pub default_path: Vec<Point2<f32>>,
}
