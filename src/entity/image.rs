use crate::component::{
  image::{ImageBundle, ImageComponent},
  position::PositionComponent,
  size::SizeComponent,
  sprite::SpriteComponent,
};
use bevy_ecs::component::ComponentId;
use macroquad::texture::load_texture;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ImageEntity {
  path: String,
  position: (f32, f32),
  ysorted: bool,
}

impl ImageEntity {
  pub async fn into(&self, index: usize) -> ImageBundle {
    let texture = load_texture(self.path.as_str()).await.unwrap();

    ImageBundle {
      image: ImageComponent {
        id: ComponentId::new(index),
      },
      position: PositionComponent {
        x: self.position.0,
        y: self.position.1,
      },
      size: SizeComponent {
        width: texture.width(),
        height: texture.height(),
      },
      sprite: SpriteComponent {
        texture,
        ysorted: self.ysorted,
      },
    }
  }
}
