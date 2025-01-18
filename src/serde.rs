use bevy::{prelude::*, tasks::IoTaskPool};
use std::{fs::File, io::Write};

// The initial scene file will be loaded below and not change when the scene is saved
const SCENE_FILE_PATH: &str = "scenes/load_scene_example.scn.ron";

pub fn deserialize(mut commands: Commands, asset_server: Res<AssetServer>) {
  commands.spawn(DynamicSceneRoot(asset_server.load(SCENE_FILE_PATH)));
}

pub fn serialize(world: &mut World) {
  let scene = DynamicScene::from_world(&world);

  // Scenes can be serialized like this:
  let type_registry = world.resource::<AppTypeRegistry>();
  let type_registry = type_registry.read();
  let serialized_scene = scene.serialize(&type_registry).unwrap();

  // Showing the scene in the console
  info!("{}", serialized_scene);

  // Writing the scene to a new file. Using a task to avoid calling the filesystem APIs in a system
  // as they are blocking
  IoTaskPool::get()
    .spawn(async move {
      // Write the scene RON data to file
      File::create(format!("assets/{SCENE_FILE_PATH}"))
        .and_then(|mut file| file.write(serialized_scene.as_bytes()))
        .expect("Error while writing scene to file");
    })
    .detach();
}
