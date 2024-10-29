use bevy::{math::bounding::Aabb2d, prelude::*};
use vleue_navigator::prelude::*;

use crate::{bounding_box::BoundingBox, obstacle::Obstacle, ysort::YSort};

#[derive(Component)]
pub struct Tree;

pub fn tree_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
  let texture = asset_server.load("tree.png");
  let x = 100.;
  let y = 100.;
  let z = 0.;

  // sprite
  commands.spawn((
    Tree,
    SpriteBundle {
      texture,
      transform: Transform::from_xyz(x, y, z),
      ..default()
    },
    YSort { height: 116 },
  ));

  // obstacle
  // TODO: child?
  commands.spawn((
    Obstacle,
    PrimitiveObstacle::Rectangle(Rectangle::new(10., 10.)),
    SpatialBundle::from_transform(Transform::from_xyz(x, y - 58., z)),
    BoundingBox {
      value: Aabb2d::new(Vec2::new(x, y - 58.), Vec2::new(5., 5.)), // TODO: Vec2::splat()
    },
  ));
}
