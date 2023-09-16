use bevy_ecs::prelude::*;
use ggez::mint::Point2;

/// View movement direction.
#[derive(PartialEq)]
pub enum ViewMovement {
  /// Left direction.
  LEFT = 0,

  /// Right direction.
  RIGHT = 1,
}

/// View component.
#[derive(Component)]
pub struct View {
  /// View points.
  pub points: Vec<Point2<f32>>,

  /// View direction angle. It means where the entity is going.
  pub direction: f32,

  /// View current direction. It means where the entity is looking.
  pub current_direction: f32,

  /// View movement.
  pub movement: ViewMovement,
}
