use bevy::{prelude::*, tasks::IoTaskPool};
use std::{fs::File, io::Write};

use crate::object::Object;

const SCENE_FILE_PATH: &str = "scenes/scene.scn.ron";

pub fn deserialize(mut commands: Commands, asset_server: Res<AssetServer>) {
  commands.spawn(DynamicSceneRoot(asset_server.load(SCENE_FILE_PATH)));
}

pub fn serialize(world: &mut World) {
  let scene = DynamicSceneBuilder::from_world(&world)
    .allow_component::<Object>()
    .build();

  let type_registry = world.resource::<AppTypeRegistry>();
  let type_registry = type_registry.read();
  let serialized_scene = scene.serialize(&type_registry).unwrap();

  IoTaskPool::get()
    .spawn(async move {
      File::create(format!("assets/{SCENE_FILE_PATH}"))
        .and_then(|mut file| file.write(serialized_scene.as_bytes()))
        .expect("Error while writing scene to file");
    })
    .detach();
}
