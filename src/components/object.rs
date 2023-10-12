use super::{position::Position, size::Size, sprite::Sprite};
use bevy_ecs::prelude::*;
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
pub struct Object {
  pub polygon: Vec<(Point2<f32>, Point2<f32>)>, // list of line segments relative to object position. must be closed polygon.
  pub polygon_type: PolygonType,
}

#[derive(Bundle)]
pub struct ObjectBundle {
  pub object: Object,
  pub position: Position,
  pub size: Size,
  pub sprite: Sprite,
}
