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
