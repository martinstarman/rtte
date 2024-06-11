use std::str::FromStr;

use bevy_ecs::{bundle::Bundle, component::Component};
use macroquad::math::Vec2;

#[derive(PartialEq, Clone, Copy)]
pub enum ShapeType {
  None = 0,        //
  Block = 1,       // blocks enemy field of view and path
  Transparent = 2, // blocks only path
  Water = 3,       //
  Snow = 4,        // blocks thath leave marks
}

impl FromStr for ShapeType {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "NONE" => Ok(ShapeType::None),
      "BLOCK" => Ok(ShapeType::Block),
      "TRASPARENT" => Ok(ShapeType::Transparent),
      "WATER" => Ok(ShapeType::Water),
      "SNOW" => Ok(ShapeType::Snow),
      _ => Err(()),
    }
  }
}

#[derive(Component, Clone)]
pub struct ShapeComponent {
  pub points: Vec<Vec2>,
  pub lines: Vec<(Vec2, Vec2)>,
  pub r#type: ShapeType,
}

#[derive(Bundle)]
pub struct ShapeBundle {
  pub shape: ShapeComponent,
}
