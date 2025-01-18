use bevy::prelude::*;
use vleue_navigator::prelude::*;

use crate::{line_of_sight::LineOfSightObstacle, ysort::YSort};

#[derive(Component, Reflect, Default)]
pub struct Object;

pub struct ObjectSpawn {
  position: Vec2,
  height: u32,
  asset_path: String,
  obstacle_position: Vec2,
  obstacle_size: Vec2,
}

impl Command for ObjectSpawn {
  fn apply(self, world: &mut World) {
    let image = world.load_asset(self.asset_path);

    world
      .spawn((
        Object,
        Sprite { image, ..default() },
        Transform::from_translation(self.position.extend(0.)),
        YSort {
          height: self.height,
        },
      ))
      .with_child((
        LineOfSightObstacle,
        Transform::from_translation(self.obstacle_position.extend(0.)),
        PrimitiveObstacle::Rectangle(Rectangle::new(self.obstacle_size.x, self.obstacle_size.y)), // TODO: polygon
      ));
  }
}

pub fn object_setup(mut commands: Commands) {
  commands.queue(ObjectSpawn {
    position: Vec2::new(100., 100.),
    height: 256,
    asset_path: String::from("objects/tree_001.png"),
    obstacle_position: Vec2::new(-5., -115.),
    obstacle_size: Vec2::new(16., 16.),
  });
}
