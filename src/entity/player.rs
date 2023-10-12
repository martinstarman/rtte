use crate::components::{
  movement::Movement,
  player::{Player, PlayerBundle},
  position::Position,
  selectable::Selectable,
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
    selectable: Selectable { selected: false },
    size: Size {
      width: 10.,
      height: 23.,
    },
    sprite: Sprite {
      image: sprite,
      y_indexed: true,
    },
  }
}
