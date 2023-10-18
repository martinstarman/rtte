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

pub fn new(index: usize, position: Position, sprite: Image) -> PlayerBundle {
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
      width: 10.,
      height: 23.,
    },
    sprite: Sprite {
      image: sprite,
      ysorted: true,
    },
  }
}
