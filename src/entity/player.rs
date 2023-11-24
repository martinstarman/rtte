use crate::component::{
  movement::MovementComponent,
  player::{PlayerBundle, PlayerComponent},
  position::PositionComponent,
  selection::SelectionComponent,
  size::SizeComponent,
  sprite::SpriteComponent,
};
use bevy_ecs::component::ComponentId;
use ggez::{graphics::Image, Context};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct PlayerEntity {
  image: String,
  position: (f32, f32),
}

impl PlayerEntity {
  pub fn into(&self, index: usize, ctx: &mut Context) -> PlayerBundle {
    let image = Image::from_path(ctx, self.image.clone()).unwrap();

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
        width: image.width() as f32,
        height: image.height() as f32,
      },
      sprite: SpriteComponent {
        image,
        ysorted: true,
      },
    }
  }
}
