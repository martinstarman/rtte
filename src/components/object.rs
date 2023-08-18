use super::{position::Position, renderable::Renderable, size::Size};
use crate::geometry::vec2::Vec2;
use bevy_ecs::prelude::*;

#[derive(Default, PartialEq, Clone, Copy)]
pub enum PolyType {
  /// Ground. It does not block anything or leave any marks.
  #[default]
  GROUND = 0,

  /// Any object that blocks enemy view and path finding (house, tree, rock, ...).
  BLOCK = 1,

  /// Any object that blocks only path finding. Like fence.
  TRANSPARENT = 2,

  /// Water.
  WATER = 3,

  /// Snow. Leave marks.
  SNOW = 4,
}

/// Object component.
#[derive(Default, Component)]
pub struct Object {
  /// Polygon. List of Vec2 relative to object position.
  _poly: Vec<Vec2>,

  /// Polygon type.
  _poly_type: PolyType,
}

/// Player bundle.
#[derive(Bundle, Default)]
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
