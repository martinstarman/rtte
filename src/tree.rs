use bevy::{math::bounding::Aabb2d, prelude::*};
use vleue_navigator::prelude::*;

use crate::{bounding_box::BoundingBox, ysort::YSort};

#[derive(Component)]
pub struct Tree;

#[derive(Component)]
pub struct Obstacle;

pub fn tree_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
  let texture = asset_server.load("tree.png");
  let x = 100.;
  let y = 100.;
  let z = 0.;

  commands.spawn((
    Tree,
    SpriteBundle {
      texture,
      transform: Transform::from_xyz(x, y, z),
      ..default()
    },
    YSort { height: 116 },
  ));

  commands.spawn((
    PrimitiveObstacle::Rectangle(Rectangle::new(10., 10.)),
    SpatialBundle::from_transform(Transform::from_xyz(x, y - 58., z)),
  ));

  // TODO: remove me
  commands.spawn((
    Obstacle,
    BoundingBox {
      value: Aabb2d::new(Vec2::new(-100., 48.), Vec2::new(5., 5.)),
    },
  ));

  commands.spawn((
    Obstacle,
    BoundingBox {
      value: Aabb2d::new(Vec2::new(-50., 5.), Vec2::new(5., 5.)),
    },
  ));
}
