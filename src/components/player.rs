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
  ///
  pub movable: Movable,

  ///
  pub player: Player,

  ///
  pub position: Position,

  ///
  pub renderable: Renderable,

  ///
  pub size: Size,

  ///
  pub selectable: Selectable,
}
