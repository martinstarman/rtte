use super::{position::PositionComponent, size::SizeComponent, sprite::SpriteComponent};
use bevy_ecs::{component::ComponentId, prelude::*};
use ggez::mint::Point2;

#[derive(PartialEq, Clone, Copy)]
pub enum PolygonType {
  GROUND = 0,      // it does not block anything or leave any marks
  BLOCK = 1,       // any object that blocks enemy view and path (house, tree, rock, ...)
  TRANSPARENT = 2, // any object that blocks only path (fence, ...)
  WATER = 3,       //
  SNOW = 4,        // leave marks
}

#[derive(Component, Clone)]
pub struct ObjectComponent {
  pub id: ComponentId,
  pub polygon: Vec<(Point2<f32>, Point2<f32>)>, // list of line segments relative to object position. must be closed polygon.
  pub polygon_type: PolygonType,
}

#[derive(Bundle)]
pub struct ObjectBundle {
  pub object: ObjectComponent,
  pub position: PositionComponent,
  pub size: SizeComponent,
  pub sprite: SpriteComponent,
}
