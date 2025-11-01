use bevy::prelude::*;
use vleue_navigator::prelude::*;

use crate::{cone_of_view::ConeOfViewObstacle, ysort::YSort};

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
  object_type: ObjectType,
  obstacle_points: Vec<Vec2>,
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
        ConeOfViewObstacle,
        Transform::from_translation(Vec3::ZERO),
        PrimitiveObstacle::ConvexPolygon(ConvexPolygon::new(self.obstacle_points).unwrap()),
      ));
    } else {
      world.spawn(component).with_child((
        Transform::from_translation(Vec3::ZERO),
        PrimitiveObstacle::ConvexPolygon(ConvexPolygon::new(self.obstacle_points).unwrap()),
      ));
    }
  }
}

pub fn object_init(mut commands: Commands) {
  commands.queue(ObjectSpawn {
    position: Vec2::new(100., 100.),
    height: 256,
    asset_path: String::from("objects/tree_001.png"),
    object_type: ObjectType::Block,
    obstacle_points: vec![
      Vec2::new(-5., -115.),
      Vec2::new(3., -115.),
      Vec2::new(3., -107.),
      Vec2::new(-5., -107.),
    ],
  });
  commands.queue(ObjectSpawn {
    position: Vec2::new(60., -70.),
    height: 81,
    asset_path: String::from("objects/fence_001.png"),
    object_type: ObjectType::SeeTrough,
    obstacle_points: vec![
      Vec2::new(-40., 0.),
      Vec2::new(20., -30.),
      Vec2::new(20., -28.),
      Vec2::new(-40., 2.),
    ],
  });
}
