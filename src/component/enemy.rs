use super::{
  movement::MovementComponent, position::PositionComponent, selection::SelectionComponent,
  size::SizeComponent, sprite::SpriteComponent, view::ViewComponent,
};
use bevy_ecs::{component::ComponentId, prelude::*};

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
  pub view: ViewComponent,
}
