use super::{
  movable::Movable, position::Position, renderable::Renderable, selectable::Selectable, size::Size,
  view::View,
};
use bevy_ecs::prelude::*;

/// Enemy component.
#[derive(Default, Component)]
pub struct Enemy {}

/// Enemy bundle.
#[derive(Bundle, Default)]
pub struct EnemyBundle {
  /// Enemy.
  pub enemy: Enemy,

  /// Enemy movement.
  pub movable: Movable,

  /// Enemy position.
  pub position: Position,

  /// Enemy rendering.
  pub renderable: Renderable,

  /// Enemy selection.
  pub selectable: Selectable,

  /// Enemy size.
  pub size: Size,

  /// Point of view.
  pub view: View,
}
