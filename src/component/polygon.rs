use bevy_ecs::{component::ComponentId, prelude::*};
use ggez::mint::Point2;

#[derive(PartialEq, Clone, Copy)]
pub enum Type {
  BLOCK = 0,       // any object that blocks enemy view and path (house, tree, rock, ...)
  TRANSPARENT = 1, // any object that blocks only path (fence, ...)
  WATER = 2,       // ...
  SNOW = 3,        // leave marks
}

#[derive(Component, Clone)]
pub struct PolygonComponent {
  pub id: ComponentId,
  pub polygon: Vec<(Point2<f32>, Point2<f32>)>,
  pub r#type: Type,
}

#[derive(Bundle)]
pub struct PolygonBundle {
  pub polygon: PolygonComponent,
}
