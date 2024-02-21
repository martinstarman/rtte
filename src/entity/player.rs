use crate::component::{
  movement::MovementComponent,
  player::{PlayerBundle, PlayerComponent},
  position::PositionComponent,
  selection::SelectionComponent,
  size::SizeComponent,
  sprite::SpriteComponent,
};
use bevy_ecs::component::ComponentId;
use macroquad::texture::load_texture;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct PlayerEntity {
  image: String,
  position: (f32, f32),
}

impl PlayerEntity {
  pub async fn into(&self, index: usize) -> PlayerBundle {
    let image = load_texture(self.image.as_str()).await.unwrap();

    PlayerBundle {
      movement: MovementComponent {
        current_path: vec![],
        default_path: vec![],
      },
      player: PlayerComponent {
        id: ComponentId::new(index),
      },
      position: PositionComponent {
        x: self.position.0,
        y: self.position.1,
      },
      selection: SelectionComponent { active: false },
      size: SizeComponent {
        width: image.width(),
        height: image.height(),
      },
      sprite: SpriteComponent {
        image,
        ysorted: true,
      },
    }
  }
}
