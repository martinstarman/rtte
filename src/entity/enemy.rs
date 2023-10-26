use crate::component::{
  enemy::{Enemy, EnemyBundle},
  movement::Movement,
  position::Position,
  selection::Selection,
  size::Size,
  sprite::Sprite,
  view::{Shift, View},
};
use bevy_ecs::component::ComponentId;
use ggez::{graphics::Image, mint::Point2};

pub fn new(
  index: usize,
  position: Position,
  image: Image,
  default_path: Vec<Point2<f32>>,
  default_direction: f32,
) -> EnemyBundle {
  EnemyBundle {
    position,
    size: Size {
      width: image.width() as f32,
      height: image.height() as f32,
    },
    sprite: Sprite {
      image,
      ysorted: true,
    },
    movement: Movement {
      current_path: default_path.clone(),
      default_path,
    },
    view: View {
      polygon: vec![],
      current_direction: default_direction,
      default_direction,
      shift: Shift::LEFT,
    },
    enemy: Enemy {
      id: ComponentId::new(index),
    },
    selection: Selection { active: false },
  }
}
