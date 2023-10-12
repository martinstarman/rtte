use super::{
  movement::Movement, position::Position, sprite::Sprite, selectable::Selectable,
  size::Size,
};
use bevy_ecs::{component::ComponentId, prelude::*};

#[derive(Component)]
pub struct Player {
  pub id: ComponentId,
}

#[derive(Bundle)]
pub struct PlayerBundle {
  pub movement: Movement,
  pub player: Player,
  pub position: Position,
  pub selectable: Selectable,
  pub size: Size,
  pub sprite: Sprite,
}
