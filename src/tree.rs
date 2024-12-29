use bevy::{math::bounding::Aabb2d, prelude::*};
use vleue_navigator::prelude::*;

use crate::{bounding_box::BoundingBox, obstacle::Obstacle, ysort::YSort};

#[derive(Component)]
pub struct Tree;

pub fn tree_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
  let image = asset_server.load("tree.png");
  let x = 100.;
  let y = 100.;
  let z = 0.;

  // sprite
  commands.spawn((
    Tree,
    Sprite { image, ..default() },
    Transform::from_xyz(x, y, z),
    YSort { height: 116 },
  ));

  // obstacle
  commands
    .spawn((
      Obstacle,
      PrimitiveObstacle::Rectangle(Rectangle::new(10., 10.)),
      Transform::from_xyz(x, y - 58., z),
    ))
    .with_children(|parent| {
      parent.spawn(BoundingBox {
        value: Aabb2d::new(Vec2::ZERO, Vec2::splat(5.)),
      });
    });
}
