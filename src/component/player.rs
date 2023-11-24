use super::{
  movement::MovementComponent, position::PositionComponent, selection::SelectionComponent,
  size::SizeComponent, sprite::SpriteComponent,
};
use bevy_ecs::{component::ComponentId, prelude::*};

#[derive(Component)]
pub struct PlayerComponent {
  pub id: ComponentId,
}

#[derive(Bundle)]
pub struct PlayerBundle {
  pub movement: MovementComponent,
  pub player: PlayerComponent,
  pub position: PositionComponent,
  pub selection: SelectionComponent,
  pub size: SizeComponent,
  pub sprite: SpriteComponent,
}
