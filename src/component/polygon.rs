use bevy_ecs::{
  bundle::Bundle,
  component::{Component, ComponentId},
};
use macroquad::math::Vec2;

#[derive(PartialEq, Clone, Copy)]
pub enum Type {
  BLOCK = 0,       // blocks enemy field of view and path
  TRANSPARENT = 1, // blocks only path
  WATER = 2,       //
  SNOW = 3,        // blocks thath leave marks
}

#[derive(Component, Clone)]
pub struct PolygonComponent {
  pub id: ComponentId,
  pub lines: Vec<(Vec2, Vec2)>,
  pub r#type: Type,
}

#[derive(Bundle)]
pub struct PolygonBundle {
  pub polygon: PolygonComponent,
}
