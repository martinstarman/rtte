use super::{
  body::BodyComponent, field_of_view::FieldOfViewComponent, movement::MovementComponent,
  position::PositionComponent, selection::SelectionComponent, size::SizeComponent,
  sprite::SpriteBundle,
};
use bevy_ecs::{
  bundle::Bundle,
  component::{Component, ComponentId},
};

#[derive(Component)]
pub struct EnemyComponent {
  pub id: ComponentId,
}

#[derive(Bundle)]
pub struct EnemyBundle {
  pub body: BodyComponent,
  pub enemy: EnemyComponent,
  pub field_of_view: FieldOfViewComponent,
  pub movement: MovementComponent,
  pub position: PositionComponent,
  pub selection: SelectionComponent,
  pub size: SizeComponent,
  pub sprite: SpriteBundle,
}
