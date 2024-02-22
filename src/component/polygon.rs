use bevy_ecs::{
  bundle::Bundle,
  component::{Component, ComponentId},
};
use macroquad::math::Vec2;

#[derive(PartialEq, Clone, Copy)]
pub enum Type {
  BLOCK = 0,       // any object that blocks enemy view and path (house, tree, rock, ...)
  TRANSPARENT = 1, // any object that blocks only path (fence, ...)
  WATER = 2,       // ...
  SNOW = 3,        // leave marks
}

#[derive(Component, Clone)]
pub struct PolygonComponent { // TODO: polygon.polygon
  pub id: ComponentId,
  pub polygon: Vec<(Vec2, Vec2)>,
  pub r#type: Type,
}

#[derive(Bundle)]
pub struct PolygonBundle {
  pub polygon: PolygonComponent,
}
