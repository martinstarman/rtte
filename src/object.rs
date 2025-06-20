use bevy::prelude::*;
use vleue_navigator::prelude::*;

use crate::{line_of_sight::LineOfSightObstacle, ysort::YSort};

#[derive(Component)]
pub struct Object;

#[derive(PartialEq, Eq)]
pub enum ObjectType {
  Block,
  SeeTrough,
}

pub struct ObjectSpawn {
  position: Vec2,
  height: u32,
  asset_path: String,
  obstacle_position: Vec2,
  obstacle_size: Vec2,
  object_type: ObjectType,
}

impl Command for ObjectSpawn {
  fn apply(self, world: &mut World) {
    let image = world.load_asset(self.asset_path);
    let component = (
      Object,
      Sprite { image, ..default() },
      Transform::from_translation(self.position.extend(0.)),
      YSort {
        height: self.height,
      },
      Pickable::IGNORE,
    );

    if self.object_type == ObjectType::Block {
      world.spawn(component).with_child((
        LineOfSightObstacle,
        Transform::from_translation(self.obstacle_position.extend(0.)),
        PrimitiveObstacle::Rectangle(Rectangle::new(self.obstacle_size.x, self.obstacle_size.y)),
      ));
    } else {
      world.spawn(component).with_child((
        Transform::from_translation(self.obstacle_position.extend(0.)),
        PrimitiveObstacle::Rectangle(Rectangle::new(self.obstacle_size.x, self.obstacle_size.y)),
      ));
    }
  }
}

pub fn object_setup(mut commands: Commands) {
  commands.queue(ObjectSpawn {
    position: Vec2::new(100., 100.),
    height: 256,
    asset_path: String::from("objects/tree_001.png"),
    obstacle_position: Vec2::new(-5., -115.),
    obstacle_size: Vec2::new(16., 16.),
    object_type: ObjectType::Block,
  });
  commands.queue(ObjectSpawn {
    position: Vec2::new(60., -70.),
    height: 81,
    asset_path: String::from("objects/fence_001.png"),
    obstacle_position: Vec2::new(-8., -20.),
    obstacle_size: Vec2::new(75., 30.),
    object_type: ObjectType::SeeTrough,
  });
}
