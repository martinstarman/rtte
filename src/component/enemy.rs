use super::{
  field_of_view::FieldOfViewComponent, movement::MovementComponent, position::PositionComponent,
  selection::SelectionComponent, size::SizeComponent, sprite::SpriteComponent,
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
  pub enemy: EnemyComponent,
  pub movement: MovementComponent,
  pub position: PositionComponent,
  pub selection: SelectionComponent,
  pub size: SizeComponent,
  pub sprite: SpriteComponent,
  pub field_of_view: FieldOfViewComponent,
}
