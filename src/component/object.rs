use bevy_ecs::{
  bundle::Bundle,
  component::{Component, ComponentId},
};

use super::{
  movement::MovementComponent, position::PositionComponent, shape::ShapeComponent,
  size::SizeComponent, sprite::SpriteBundle,
};

#[derive(Component, Clone)]
pub struct ObjectComponent {
  pub id: ComponentId,
}

#[derive(Bundle)]
pub struct ObjectBundle {
  // pub body: BodyComponent,
  pub movement: MovementComponent,
  pub object: ObjectComponent,
  pub position: PositionComponent,
  pub shape: ShapeComponent,
  pub size: SizeComponent,
  pub sprite: SpriteBundle,
}
