use super::{
  movable::Movable, position::Position, renderable::Renderable, selectable::Selectable, size::Size,
};
use bevy_ecs::{prelude::*, component::ComponentId};

/// Player component.
#[derive(Component)]
pub struct Player {
  /// Player id.
  pub id: ComponentId,
}

/// Player bundle.
#[derive(Bundle)]
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
