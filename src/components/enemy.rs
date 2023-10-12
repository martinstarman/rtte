use super::{
  movement::Movement, position::Position, selectable::Selectable, size::Size, sprite::Sprite,
  view::View,
};
use bevy_ecs::{component::ComponentId, prelude::*};

#[derive(Component)]
pub struct Enemy {
  pub id: ComponentId,
}

#[derive(Bundle)]
pub struct EnemyBundle {
  pub enemy: Enemy,
  pub movement: Movement,
  pub position: Position,
  pub selectable: Selectable,
  pub size: Size,
  pub sprite: Sprite,
  pub view: View,
}
