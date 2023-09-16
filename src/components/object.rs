use super::{position::Position, renderable::Renderable, size::Size};
use bevy_ecs::prelude::*;
use ggez::mint::Point2;

/// Polygon types.
#[derive(PartialEq, Clone, Copy)]
pub enum PolygonType {
  /// Ground. It does not block anything or leave any marks.
  GROUND = 0,

  /// Any object that blocks enemy view and path. Like house, tree, rock, etc.
  BLOCK = 1,

  /// Any object that blocks only path. Like fence.
  TRANSPARENT = 2,

  /// Water.
  WATER = 3,

  /// Snow. Leave marks.
  SNOW = 4,
}

/// Object component.
#[derive(Component, Clone)]
pub struct Object {
  /// List of points relative to object position.
  pub polygon: Vec<Point2<f32>>,

  /// Polygon type.
  pub polygon_type: PolygonType,
}

/// Player bundle.
#[derive(Bundle)]
pub struct ObjectBundle {
  /// Object.
  pub object: Object,

  /// Object position.
  pub position: Position,

  /// Object rendering.
  pub renderable: Renderable,

  /// Player size.
  pub size: Size,
}
