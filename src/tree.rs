use bevy::prelude::*;
use vleue_navigator::prelude::*;

use crate::{line_of_sight::LineOfSightObstacle, ysort::YSort};

// TODO: make it universal (rename to object?)
#[derive(Component)]
pub struct Tree;

pub fn tree_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
  let image = asset_server.load("tree.png");
  let x = 100.;
  let y = 100.;
  let z = 0.;

  // sprite
  commands
    .spawn((
      Tree,
      Sprite { image, ..default() },
      Transform::from_xyz(x, y, z),
      YSort { height: 116 },
    ))
    .with_children(|parent| {
      parent.spawn((
        LineOfSightObstacle,
        Transform::from_xyz(0., -58., 0.),
        PrimitiveObstacle::Rectangle(Rectangle::new(10., 10.)),
      ));
    });
}
