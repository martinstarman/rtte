use bevy_ecs::prelude::*;
use ggez::mint::Point2;

/// View component.
#[derive(Component)]
pub struct View {
  /// View points.
  pub points: Vec<Point2<f32>>,

  /// View x position.
  pub x: f32,

  /// View y position.
  pub y: f32,
}
