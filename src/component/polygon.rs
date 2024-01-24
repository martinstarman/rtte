use bevy_ecs::{component::ComponentId, prelude::*};
use ggez::mint::Point2;

#[derive(PartialEq, Clone, Copy)]
pub enum Type {
  GROUND = 0,      // it does not block anything or leave any marks
  BLOCK = 1,       // any object that blocks enemy view and path (house, tree, rock, ...)
  TRANSPARENT = 2, // any object that blocks only path (fence, ...)
  WATER = 3,       //
  SNOW = 4,        // leave marks
}

#[derive(Component, Clone)]
pub struct PolygonComponent {
  pub id: ComponentId,
  pub polygon: Vec<(Point2<f32>, Point2<f32>)>, // TODO: Vec<points>?
  pub r#type: Type,
}

#[derive(Bundle)]
pub struct PolygonBundle {
  pub polygon: PolygonComponent,
}
