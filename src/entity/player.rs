use crate::component::{
  movement::Movement,
  player::{Player, PlayerBundle},
  position::Position,
  selection::Selection,
  size::Size,
  sprite::Sprite,
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
  pub fn to_component(&self, index: usize, ctx: &mut Context) -> PlayerBundle {
    let image = Image::from_path(ctx, self.image.clone()).unwrap();

    PlayerBundle {
      movement: Movement {
        current_path: vec![],
        default_path: vec![],
      },
      player: Player {
        id: ComponentId::new(index),
      },
      position: Position {
        x: self.position.0,
        y: self.position.1,
      },
      selection: Selection { active: false },
      size: Size {
        width: image.width() as f32,
        height: image.height() as f32,
      },
      sprite: Sprite {
        image,
        ysorted: true,
      },
    }
  }
}
