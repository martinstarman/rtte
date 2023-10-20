use crate::components::{
  movement::Movement,
  player::{Player, PlayerBundle},
  position::Position,
  selection::Selection,
  size::Size,
  sprite::Sprite,
};
use bevy_ecs::component::ComponentId;
use ggez::graphics::Image;

pub fn new(index: usize, position: Position, image: Image) -> PlayerBundle {
  PlayerBundle {
    movement: Movement {
      current_path: vec![],
      default_path: vec![],
    },
    player: Player {
      id: ComponentId::new(index),
    },
    position,
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
