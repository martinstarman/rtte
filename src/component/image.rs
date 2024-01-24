use super::{position::PositionComponent, size::SizeComponent, sprite::SpriteComponent};
use bevy_ecs::{component::ComponentId, prelude::*};

#[derive(Component)]
pub struct ImageComponent {
  pub id: ComponentId,
}

#[derive(Bundle)]
pub struct ImageBundle {
  pub image: ImageComponent,
  pub position: PositionComponent,
  pub size: SizeComponent,
  pub sprite: SpriteComponent,
}
