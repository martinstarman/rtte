use bevy_ecs::{
  bundle::Bundle,
  component::{Component, ComponentId},
};

use super::{
  position::PositionComponent, shape::ShapeComponent, size::SizeComponent, sprite::SpriteComponent,
};

#[derive(Component, Clone)]
pub struct ObjectComponent {
  pub id: ComponentId,
}

#[derive(Bundle)]
pub struct ObjectBundle {
  pub object: ObjectComponent,
  pub position: PositionComponent,
  pub sprite: SpriteComponent,
  pub shape: ShapeComponent,
  pub size: SizeComponent,
}
