use crate::component::{
  image::{ImageBundle, ImageComponent},
  position::PositionComponent,
  size::SizeComponent,
  sprite::SpriteComponent,
};
use bevy_ecs::component::ComponentId;
use ggez::{graphics::Image, Context};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ImageEntity {
  path: String,
  position: (f32, f32),
  ysorted: bool,
}

impl ImageEntity {
  pub fn into(&self, index: usize, ctx: &mut Context) -> ImageBundle {
    let image = Image::from_path(ctx, self.path.clone()).unwrap();

    ImageBundle {
      image: ImageComponent {
        id: ComponentId::new(index),
      },
      position: PositionComponent {
        x: self.position.0,
        y: self.position.1,
      },
      size: SizeComponent {
        width: image.width() as f32,
        height: image.height() as f32,
      },
      sprite: SpriteComponent {
        image,
        ysorted: self.ysorted,
      },
    }
  }
}
