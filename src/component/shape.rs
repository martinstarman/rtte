use bevy_ecs::{bundle::Bundle, component::Component};
use macroquad::math::Vec2;

#[derive(PartialEq, Clone, Copy)]
pub enum ShapeType {
  NONE = 0,        //
  BLOCK = 1,       // blocks enemy field of view and path
  TRANSPARENT = 2, // blocks only path
  WATER = 3,       //
  SNOW = 4,        // blocks thath leave marks
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
