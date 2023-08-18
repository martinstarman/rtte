use super::{
  movable::Movable, position::Position, renderable::Renderable, selectable::Selectable, size::Size,
};
use bevy_ecs::prelude::*;

/// Player component.
#[derive(Default, Component)]
pub struct Player {}

/// Player bundle.
#[derive(Bundle, Default)]
pub struct PlayerBundle {
  /// Player movement.
  pub movable: Movable,

  /// Player.
  pub player: Player,

  /// Player position.
  pub position: Position,

  /// Player rendering.
  pub renderable: Renderable,

  /// Player selection.
  pub selectable: Selectable,

  /// Player size.
  pub size: Size,
}