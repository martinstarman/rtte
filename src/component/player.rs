use super::{
  body::BodyComponent, movement::MovementComponent, position::PositionComponent,
  selection::SelectionComponent, size::SizeComponent, sprite::SpriteBundle,
};
use bevy_ecs::{
  bundle::Bundle,
  component::{Component, ComponentId},
};

#[derive(Component)]
pub struct PlayerComponent {
  pub id: ComponentId,
}

#[derive(Bundle)]
pub struct PlayerBundle {
  pub body: BodyComponent,
  pub movement: MovementComponent,
  pub player: PlayerComponent,
  pub position: PositionComponent,
  pub selection: SelectionComponent,
  pub size: SizeComponent,
  pub sprite: SpriteBundle,
}
